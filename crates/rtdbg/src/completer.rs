use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use librtdbg::os::{NativeRuntimeProvider, runtime::RuntimeProvider};
use reedline::{Completer, CompletionResult, Span, Suggestion};

use crate::{
    repl::ATTACHED_PID,
    socket::{get_rpcs, interact_remote, remote_scripts},
};

pub struct RtdbgCompleter {
    cmds: Vec<String>,
}

impl Completer for RtdbgCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> CompletionResult {
        let head = &line[..pos];
        let start = head.rfind(char::is_whitespace).map(|i| i + 1).unwrap_or(0);
        let span = Span { start, end: pos };

        if let Some((pid_str, rest)) = line.trim_start().split_once(' ')
            && let Some(pid) = pid_str.parse::<u32>().ok()
        {
            return self.complete(rest, pos - pid_str.len() - 1, span, Some(pid));
        }

        self.complete(line, pos, span, None)
    }
}

impl RtdbgCompleter {
    pub fn new(cmds: Vec<String>) -> RtdbgCompleter {
        RtdbgCompleter { cmds }
    }

    fn complete(
        &mut self,
        line: &str,
        pos: usize,
        span: Span,
        mut pid: Option<u32>,
    ) -> CompletionResult {
        let head = &line[..pos];
        let start = head.rfind(char::is_whitespace).map(|i| i + 1).unwrap_or(0);
        let prefix = &head[start..];

        let pid_passed_in = pid.is_some();

        if pid.is_none()
            && let Ok(global_pid) = ATTACHED_PID.read().as_deref()
        {
            pid = *global_pid;
        }

        if head[..start].trim().is_empty() {
            // Command completion
            let mut suggestions = self
                .cmds
                .iter()
                .filter(|c| c.to_string().starts_with(prefix))
                .map(|c| Suggestion {
                    value: c.clone(),
                    span,
                    append_whitespace: true,
                    ..Default::default()
                })
                .collect::<Vec<Suggestion>>();

            // Complete PIDs
            if !pid_passed_in {
                suggestions.extend(complete_relevant_pid(prefix, span));
            }

            // Complete script ids
            if let Some(pid) = pid {
                suggestions.extend(complete_script_ids(pid, false, prefix, span));

                // Complete RPCs
                if prefix.contains('.') && prefix.contains('#') {
                    suggestions.extend(complete_rpcs(pid, prefix, span));
                }
            }

            return CompletionResult::Fresh {
                suggestions: suggestions.into(),
                partial: None,
            };
        }

        let mut words = line.split_whitespace();

        let cmd = words.next().unwrap();

        let arg_pos = line.rsplit_once(' ').into_iter().count();

        match (cmd, arg_pos, pid) {
            ("load", 1, Some(_)) => CompletionResult::Fresh {
                suggestions: complete_path(prefix, span, Some(".rhai")).into(),
                partial: None,
            },
            ("unload", 1, Some(pid)) => CompletionResult::Fresh {
                suggestions: complete_script_ids(pid, true, prefix, span).into(),
                partial: None,
            },
            ("launch", 1, _) => CompletionResult::Fresh {
                suggestions: complete_path(prefix, span, None).into(),
                partial: None,
            },
            ("inject", 1, _) => CompletionResult::Pending,
            (_, 1, Some(pid)) if prefix.contains('#') && prefix.contains('.') => {
                CompletionResult::Fresh {
                    suggestions: complete_rpcs(pid, prefix, span).into(),
                    partial: None,
                }
            }
            (_, 1, Some(pid)) => CompletionResult::Fresh {
                suggestions: complete_script_ids(pid, false, prefix, span).into(),
                partial: None,
            },
            _ => CompletionResult::Pending,
        }
    }
}

fn complete_path(prefix: &str, span: Span, ext: Option<&str>) -> Vec<Suggestion> {
    let expanded = if let Some(rest) = prefix.strip_prefix("~/") {
        home::home_dir()
            .map(|h| h.join(rest))
            .unwrap_or_else(|| prefix.into())
    } else {
        PathBuf::from(prefix)
    };

    let (dir, partial) = if prefix.is_empty() || prefix.ends_with('/') {
        (expanded.clone(), String::new())
    } else {
        (
            expanded.parent().map(Path::to_path_buf).unwrap_or_default(),
            expanded
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default(),
        )
    };
    let dir = if dir.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        dir
    };

    let keep = prefix.rfind('/').map(|i| &prefix[..=i]).unwrap_or("");

    let Ok(rd) = std::fs::read_dir(&dir) else {
        return vec![];
    };
    let mut out: Vec<Suggestion> = rd
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            if !name.starts_with(&partial) {
                return None;
            }
            if !partial.starts_with('.') && name.starts_with('.') {
                return None;
            }
            let is_dir = e.file_type().ok()?.is_dir();
            if !is_dir && ext.map_or(false, |x| !name.ends_with(x)) {
                return None;
            }

            let mut value = format!("{keep}{name}");
            if is_dir {
                value.push('/');
            }
            Some(Suggestion {
                value,
                span,
                append_whitespace: !is_dir,
                ..Default::default()
            })
        })
        .collect();
    out.sort_by(|a, b| a.value.cmp(&b.value));
    out
}

fn complete_script_ids(pid: u32, append_space: bool, prefix: &str, span: Span) -> Vec<Suggestion> {
    let scripts = interact_remote(pid, remote_scripts);

    let prefix = prefix.strip_prefix("#").unwrap_or(prefix);

    if let Ok(scripts) = scripts {
        return scripts
            .into_iter()
            .filter(|(id, _)| id.0.to_string().starts_with(prefix))
            .map(|(id, _)| Suggestion {
                value: format!("#{}", id.0),
                span,
                append_whitespace: append_space,
                ..Default::default()
            })
            .collect();
    }

    Vec::new()
}

fn complete_relevant_pid(prefix: &str, span: Span) -> Vec<Suggestion> {
    let rtdbg_path = NativeRuntimeProvider::rtdbg_path().expect("No rtdbg runtime path");

    std::fs::read_dir(rtdbg_path)
        .expect("Failed to read rtdbg path")
        .into_iter()
        .flatten()
        .map(|e| e.path())
        .filter(|e| {
            e.extension()
                .and_then(OsStr::to_str)
                .is_some_and(|e| e.eq_ignore_ascii_case("sock"))
        })
        .map(|e| {
            e.file_stem()
                .map(|e| e.to_str())
                .flatten()
                .map(ToString::to_string)
                .filter(|e| e.starts_with(prefix))
        })
        .flatten()
        .filter(|e| {
            if let Ok(val) = e.parse::<u32>() {
                !NativeRuntimeProvider::prune_inactive_socket(val)
            } else {
                false
            }
        })
        .map(|e| Suggestion {
            value: e,
            span,
            append_whitespace: true,
            ..Default::default()
        })
        .collect()
}

pub fn rpc_values(pid: u32) -> Vec<(String, Vec<String>)> {
    interact_remote(pid, get_rpcs)
        .into_iter()
        .flatten()
        .map(|(script_id, rpcs)| {
            rpcs.into_iter()
                .map(|(rpc_name, params)| (format!("#{}.{}(", script_id.0, rpc_name), params))
                .collect::<Vec<(String, Vec<String>)>>()
        })
        .flatten()
        .collect()
}

fn complete_rpcs(pid: u32, prefix: &str, span: Span) -> Vec<Suggestion> {
    rpc_values(pid)
        .into_iter()
        .filter(|(value, _)| value.starts_with(prefix))
        .map(|(value, _)| Suggestion {
            value,
            span,
            append_whitespace: false,
            ..Default::default()
        })
        .collect()
}
