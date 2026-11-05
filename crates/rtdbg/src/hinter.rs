use reedline::Hinter;

use crate::{argkind::ArgKind, completer::rpc_values, repl::ATTACHED_PID};

pub struct RtdbgHinter {}

impl Hinter for RtdbgHinter {
    fn handle(
        &mut self,
        mut line: &str,
        _: usize,
        _: &dyn reedline::History,
        _: bool,
        _: &str,
    ) -> String {
        let mut pid = ATTACHED_PID.read().as_deref().ok().copied().flatten();

        if let Some((pid_maybe, rest)) = line.trim_start().split_once(' ') {
            line = rest;
            pid = pid_maybe.parse::<u32>().ok();
        }

        let Some(pid) = pid else {
            return String::new();
        };

        let Some((_, params)) = rpc_values(pid).into_iter().find(|(rpc_value, _)| {
            if let Some(prefix_removed) = line.trim_start().strip_prefix(rpc_value) {
                line = prefix_removed;

                true
            } else {
                false
            }
        }) else {
            return String::new();
        };

        if let Ok((parsed_args, rest)) = ArgKind::parse_partial(line) {
            let mut arg_count = parsed_args.len();

            if !rest.is_empty() {
                arg_count += 1;
            }

            let hint = params
                .into_iter()
                .skip(arg_count)
                .collect::<Vec<String>>()
                .join(", ");

            let hint = if hint.is_empty() || (rest.is_empty() && arg_count == 0) {
                hint
            } else if line.ends_with(',') {
                format!(" {}", hint)
            } else {
                format!(", {}", hint)
            };

            let mut final_hint = "\x1b[2m".to_string();

            final_hint.push_str(&hint);

            if !rest.trim().ends_with(')') {
                final_hint.push(')');
            }

            final_hint.push_str("\x1b[0m");

            return final_hint;
        }

        String::new()
    }

    fn complete_hint(&self) -> String {
        String::new()
    }

    fn next_hint_token(&self) -> String {
        String::new()
    }
}
