use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, RwLock},
};

use color_eyre::eyre::Result;
use librtdbg::script::ScriptId;
use reedline::{
    ColumnarMenu, DefaultPrompt, Emacs, KeyCode, KeyModifiers, MenuBuilder, Reedline,
    ReedlineEvent, ReedlineMenu, Signal, default_emacs_keybindings,
};
use serde_json::Value;
use thiserror::Error;

use crate::{
    argkind::ArgKind,
    completer::RtdbgCompleter,
    hinter::RtdbgHinter,
    launch,
    socket::{interact_remote, load_script, run_rpc, unload_script},
};

pub static ATTACHED_PID: LazyLock<Arc<RwLock<Option<u32>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(None)));

pub enum Command {
    Load(Option<u32>, PathBuf),
    Unload(Option<u32>, ScriptId),
    Launch(PathBuf),
    Inject(u32),

    Rpc(Option<u32>, Rpc),
}

pub struct Rpc {
    script_id: ScriptId,
    name: String,
    args: Vec<ArgKind>,
}

pub fn init() -> Result<()> {
    let rtdbg_completer = RtdbgCompleter::new(vec![
        "load".to_string(),
        "unload".to_string(),
        "launch".to_string(),
        "inject".to_string(),
    ]);

    let rtdbg_hinter = RtdbgHinter {};

    let completion_menu = ColumnarMenu::default().with_name("completion_menu");

    let mut kb = default_emacs_keybindings();

    kb.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".into()),
            ReedlineEvent::MenuNext,
        ]),
    );

    let line_editor = Reedline::create()
        .with_ansi_colors(true)
        .with_hinter(Box::new(rtdbg_hinter))
        .with_completer(Box::new(rtdbg_completer))
        .with_menu(ReedlineMenu::EngineCompleter(Box::new(completion_menu)))
        .with_edit_mode(Box::new(Emacs::new(kb)));
    let prompt = DefaultPrompt::default();

    repl(line_editor, prompt)
}

fn repl(mut line_editor: Reedline, prompt: DefaultPrompt) -> Result<()> {
    loop {
        let sig = line_editor.read_line(&prompt);

        match sig {
            Ok(Signal::Success(buffer)) => {
                let parsed_command = match parse_command(buffer) {
                    Ok(cmd) => cmd,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };

                let result = match parsed_command {
                    Command::Load(Some(pid), path_buf) => handle_load(pid, &path_buf),
                    Command::Unload(Some(pid), script_id) => handle_unload(pid, script_id),
                    Command::Launch(path_buf) => handle_launch(&path_buf),
                    Command::Inject(_pid) => {
                        eprintln!("Not yet implemented!");
                        continue;
                    }
                    Command::Rpc(Some(pid), rpc) => handle_rpc(pid, rpc),
                    _ => {
                        eprintln!("No PID provided and no process attached!");
                        continue;
                    }
                };

                if let Err(e) = result {
                    eprint!("{}", e);
                }
            }
            Ok(Signal::CtrlC) | Ok(Signal::CtrlD) => {
                println!("\nGoodbye!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }

    Ok(())
}

fn handle_load(pid: u32, path: &Path) -> Result<(), Error> {
    let mut file = File::open(path)?;

    let mut script_contents = String::new();

    file.read_to_string(&mut script_contents)?;

    let id = interact_remote(pid, |socket| load_script(socket, script_contents))?;

    println!("Loaded #{}", id.0);

    Ok(())
}

fn handle_unload(pid: u32, script_id: ScriptId) -> Result<(), Error> {
    interact_remote(pid, |socket| unload_script(socket, script_id))?;

    println!("Success!");

    Ok(())
}

fn handle_launch(path: &Path) -> Result<(), Error> {
    launch::launch(path)?;

    Ok(())
}

fn handle_rpc(pid: u32, rpc: Rpc) -> Result<(), Error> {
    let args = rpc
        .args
        .into_iter()
        .map(|a| Ok(a.try_into()?))
        .collect::<Result<Vec<Value>, Error>>()?;

    let resp = interact_remote(pid, |socket| {
        Ok(run_rpc(
            socket,
            &rpc.script_id,
            rpc.name,
            serde_json::to_value(args)?,
        ))
    })??;

    println!("{}", resp);

    Ok(())
}

fn parse_command(cmd: String) -> Result<Command, Error> {
    let mut cmd = cmd.trim();

    let mut pid = *ATTACHED_PID.read().expect("ATTACHED_PID poisoned");

    if let Some((pid_maybe, rest)) = cmd.trim_start().split_once(' ')
        && let Ok(pid_inline) = pid_maybe.parse::<u32>()
    {
        pid = Some(pid_inline);
        cmd = rest;
    }

    if let Some(cmd) = cmd.strip_prefix("load") {
        return parse_load(pid, cmd);
    }

    if let Some(cmd) = cmd.strip_prefix("unload") {
        return parse_unload(pid, cmd);
    }

    if let Some(cmd) = cmd.strip_prefix("launch") {
        return parse_launch(cmd);
    }

    if let Some(cmd) = cmd.strip_prefix("inject") {
        return parse_inject(cmd);
    }

    if let Ok(rpc) = parse_rpc(pid, cmd) {
        return Ok(rpc);
    }

    Err(Error::UnknownCommand)
}

fn parse_load(pid: Option<u32>, args: &str) -> Result<Command, Error> {
    let args = ArgKind::parse(args)?;

    if args.len() != 1 {
        return Err(Error::IncorrectArgCount(1));
    }

    let path = match &args[0] {
        ArgKind::String(str) => PathBuf::from(str.clone()),
        ArgKind::Garbage(garb) => PathBuf::from(garb.clone()),
        _ => return Err(Error::IncorrectArgumentPos("Path".to_string(), 1)),
    };

    Ok(Command::Load(pid, path))
}

fn parse_unload(pid: Option<u32>, args: &str) -> Result<Command, Error> {
    let args = ArgKind::parse(args)?;

    if args.len() != 1 {
        return Err(Error::IncorrectArgCount(1));
    }

    let script_id = match &args[0] {
        ArgKind::ScriptId(id) => *id,
        _ => return Err(Error::IncorrectArgumentPos("ScriptId".to_string(), 1)),
    };

    Ok(Command::Unload(pid, ScriptId(script_id)))
}

fn parse_launch(args: &str) -> Result<Command, Error> {
    let args = ArgKind::parse(args)?;

    if args.len() != 1 {
        return Err(Error::IncorrectArgCount(1));
    }

    let path = match &args[0] {
        ArgKind::String(str) => PathBuf::from(str.clone()),
        ArgKind::Garbage(garb) => PathBuf::from(garb.clone()),
        _ => return Err(Error::IncorrectArgumentPos("Path".to_string(), 1)),
    };

    Ok(Command::Launch(path))
}

fn parse_inject(args: &str) -> Result<Command, Error> {
    let args = ArgKind::parse(args)?;

    if args.len() != 1 {
        return Err(Error::IncorrectArgCount(1));
    }

    let pid: u32 = match &args[0] {
        ArgKind::Int(int) => int
            .clone()
            .try_into()
            .map_err(|_| Error::IncorrectArgumentPos("Pid".to_string(), 1))?,
        _ => return Err(Error::IncorrectArgumentPos("Pid".to_string(), 1)),
    };

    Ok(Command::Inject(pid))
}

fn parse_rpc(pid: Option<u32>, cmd: &str) -> Result<Command, Error> {
    let Some((script_id, rest)) = cmd.split_once('.') else {
        return Err(Error::MalformedRpc);
    };

    let script_id_arg = ArgKind::parse_sub(script_id.trim());

    let script_id = match script_id_arg {
        Ok(ArgKind::ScriptId(id)) => id,
        _ => return Err(Error::IncorrectArgument("ScriptId".to_string())),
    };

    let Some((rpc_name, args)) = rest.split_once('(') else {
        return Err(Error::MalformedRpc);
    };

    let args = args.strip_suffix(')').unwrap_or(args);

    let args_parsed = ArgKind::parse(args)?;

    Ok(Command::Rpc(
        pid,
        Rpc {
            script_id: ScriptId(script_id),
            name: rpc_name.to_string(),
            args: args_parsed,
        },
    ))
}

#[derive(Error, Debug)]
pub enum Error {
    // ArgKind errors
    // #[error("Unexpected character '{0}' at {1}")]
    // UnexpectedCharacter(char, usize),
    #[error("Unexpected end of array at {0}")]
    MismatchedParens(usize),
    // #[error("'{0}' is not a valid ArgType")]
    // InvalidArgType(String),
    #[error("Input is invalid")]
    InvalidInput,
    #[error("Refused to serialise invalid ArgKind")]
    RefusingToSerialiseGarbage,

    #[error("Unrecognised command")]
    UnknownCommand,
    #[error("Expected {0} arguments")]
    IncorrectArgCount(u8),
    #[error("Expected {0} at arg pos {1}")]
    IncorrectArgumentPos(String, u8),
    #[error("Expected {0}")]
    IncorrectArgument(String),
    #[error("Rpc invocation is malformed")]
    MalformedRpc,

    // Other
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    RtdbgError(#[from] librtdbg::error::Error),
    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),
}
