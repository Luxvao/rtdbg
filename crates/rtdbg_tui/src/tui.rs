use std::{
    collections::VecDeque,
    fmt::Display,
    os::unix::net::UnixStream,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use librtdbg::script::ScriptId;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, List, ListState, Paragraph, Tabs},
};
use sysinfo::{Pid, System};

use crate::{
    components::{MultiLineTextInput, TextInput, TextInputStatus, render_error, update_error},
    inject::launch,
    script::load_from_disk,
    socket::{load_script, remote_scripts, unload_script},
};

pub fn init() -> Result<()> {
    ratatui::run(|t| App::app(&mut App::default(), t))?;

    Ok(())
}

#[derive(Debug, Default)]
enum InjectionStatus {
    #[default]
    NotInjected,
    LdPreload,
    Ptrace,
}

impl Display for InjectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Injection method:\n{}",
            match self {
                InjectionStatus::NotInjected => "Not injected",
                InjectionStatus::LdPreload => "LD_PRELOAD",
                InjectionStatus::Ptrace => "ptrace",
            }
        )
    }
}

#[derive(Default, Debug)]
enum State {
    #[default]
    Normal,
    ScriptPreview,
    DisplayError(String),
    PidTextInput(TextInput),
    LaunchTextInput(TextInput),
    AddScriptTextInput(TextInput),
}

#[derive(Default, Debug, Copy, Clone)]
enum CurrentTab {
    #[default]
    Overview = 0,
    Scripts = 1,
    Rpc = 2,
    Hooks = 3,
}

#[derive(Default, Debug)]
pub struct ScriptsManager {
    local_selected: bool,

    remote_scripts: Vec<(ScriptId, String)>,
    pub local_scripts: Vec<(String, String)>,

    remote_scripts_state: ListState,
    local_scripts_state: ListState,

    preview: MultiLineTextInput,
}

impl ScriptsManager {
    fn get_sel_len(&self) -> usize {
        if self.local_selected {
            return self.local_scripts.len();
        }

        self.remote_scripts.len()
    }

    fn get_src_at(&self, index: usize) -> Option<String> {
        if self.local_selected {
            return self.local_scripts.get(index).map(|s| s.1.clone());
        }

        self.remote_scripts.get(index).map(|s| s.1.clone())
    }

    fn set_src_at(&mut self, index: usize, new_src: String) {
        if self.local_selected {
            if let Some(src) = self.local_scripts.get_mut(index) {
                src.1 = new_src;
            }

            return;
        }

        if let Some(src) = self.remote_scripts.get_mut(index) {
            src.1 = new_src;
        }
    }

    fn toggle_active(&mut self) {
        self.local_selected = !self.local_selected;
    }

    fn get_state(&mut self) -> &mut ListState {
        if self.local_selected {
            &mut self.local_scripts_state
        } else {
            &mut self.remote_scripts_state
        }
    }

    fn update_preview_perms(&mut self) {
        if self.local_selected {
            self.preview.read_only(false);
        } else {
            self.preview.read_only(true);
        }
    }

    fn get_remote_title(&self) -> &'static str {
        if self.local_selected {
            "Injected"
        } else {
            "* Injected"
        }
    }

    fn get_local_title(&self) -> &'static str {
        if self.local_selected {
            "* Local"
        } else {
            "Local"
        }
    }

    fn get_binds(&self) -> &'static str {
        if self.local_selected {
            " [tab] switch list │ [i]nject │ [a]dd │ [r]emove"
        } else {
            " [tab] switch list │ [u]nload"
        }
    }
}

#[derive(Default, Debug)]
pub struct App {
    exit: bool,
    tab: CurrentTab,
    state: State,

    // Process stuff
    pid: Option<u32>,
    injection_method: InjectionStatus,
    stdout_scroll: usize,
    stdout_paragraph_height: usize,
    stdout: Arc<Mutex<VecDeque<String>>>,
    socket: Option<UnixStream>,

    // Scripts
    pub scripts_mgr: ScriptsManager,
}

impl App {
    pub fn app(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;

            if self.exit {
                break Ok(());
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let main_block = Block::bordered().title("RTDBG TUI");

        frame.render_widget(&main_block, frame.area());

        let [tabs_area, content] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .areas(main_block.inner(frame.area()));

        let tabs = Tabs::new(vec!["[1] Overview", "[2] Scripts", "[3] RPC", "[4] Hooks"])
            .select(self.tab as usize);

        frame.render_widget(tabs, tabs_area);

        match self.tab {
            CurrentTab::Overview => self.construct_overview(frame, content),
            CurrentTab::Scripts => self.construct_script(frame, content),
            _ => (),
        }

        match self.state {
            State::Normal => (),
            State::ScriptPreview => (),
            State::DisplayError(ref e) => render_error(e, frame, frame.area()),
            State::PidTextInput(ref ti) => ti.render(frame, frame.area()),
            State::LaunchTextInput(ref ti) => ti.render(frame, frame.area()),
            State::AddScriptTextInput(ref ti) => ti.render(frame, frame.area()),
        }
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if !event::poll(Duration::from_millis(10))? {
            return Ok(());
        }

        match event::read()? {
            Event::Key(k) => match k.code {
                // Display error
                a if let State::DisplayError(_) = self.state => {
                    if update_error(a) {
                        self.state = State::Normal;
                    }
                }

                // Pid input (inject)
                a if let State::PidTextInput(ref mut ti) = self.state => match ti.update(a) {
                    TextInputStatus::Cancelled => self.state = State::Normal,
                    TextInputStatus::Continue => (),
                    TextInputStatus::Done(i) => {
                        self.state = State::Normal;
                    }
                },
                // Launch input
                a if let State::LaunchTextInput(ref mut ti) = self.state => match ti.update(a) {
                    TextInputStatus::Cancelled => self.state = State::Normal,
                    TextInputStatus::Continue => (),
                    TextInputStatus::Done(i) => {
                        self.state = State::Normal;

                        let (stream, pid) = match launch(&i, self.stdout.clone()) {
                            Ok(v) => v,
                            Err(e) => {
                                self.state =
                                    State::DisplayError(format!("Unable to launch, error: {}", e));
                                return Ok(());
                            }
                        };

                        self.socket = Some(stream);
                        self.pid = Some(pid);
                        self.injection_method = InjectionStatus::LdPreload;
                    }
                },
                // Add script input
                a if let State::AddScriptTextInput(ref mut ti) = self.state => match ti.update(a) {
                    TextInputStatus::Cancelled => self.state = State::Normal,
                    TextInputStatus::Continue => (),
                    TextInputStatus::Done(i) => {
                        self.state = State::Normal;

                        let path = PathBuf::from(i);

                        match load_from_disk(&path) {
                            Ok(v) => self.scripts_mgr.local_scripts.push(v),
                            Err(e) => {
                                self.state = State::DisplayError(format!(
                                    "Unable to add script, error: {}",
                                    e
                                ))
                            }
                        }
                    }
                },

                // Preview editor
                a if let CurrentTab::Scripts = self.tab
                    && let State::ScriptPreview = self.state =>
                {
                    if self.scripts_mgr.preview.update(a) {
                        self.state = State::Normal;
                    }
                }

                // Exit
                KeyCode::Char('q') => self.exit = true,

                // Tabs
                KeyCode::Char('1') => self.tab = CurrentTab::Overview,
                KeyCode::Char('2') => {
                    if let Some(ref mut socket) = self.socket {
                        match remote_scripts(socket) {
                            Ok(scripts) => self.scripts_mgr.remote_scripts = scripts,
                            Err(e) => {
                                self.state = State::DisplayError(format!(
                                    "Unable to fetch remote scripts, error: {}",
                                    e
                                ))
                            }
                        }
                    }

                    self.tab = CurrentTab::Scripts;
                }
                KeyCode::Char('3') => self.tab = CurrentTab::Rpc,
                KeyCode::Char('4') => self.tab = CurrentTab::Hooks,

                // Overview
                a if let CurrentTab::Overview = self.tab => self.handle_events_overview(a),

                // Scripts
                a if let CurrentTab::Scripts = self.tab => self.handle_events_scripts(a),

                _ => (),
            },
            _ => (),
        }

        Ok(())
    }

    fn construct_overview(&mut self, frame: &mut Frame, area: Rect) {
        let [left, right] =
            Layout::horizontal([Constraint::Fill(3), Constraint::Fill(1)]).areas(area);

        // Stdout & binds
        // Layout
        let [stdout_top, binds_bottom] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(left);

        // Stdout
        let stdout_paragraph_block = Block::bordered().title("Target output");

        let stdout_lock = self.stdout.try_lock();

        // Stdout paragraph height
        let stdout_height = stdout_paragraph_block.inner(stdout_top).height as usize;

        self.stdout_paragraph_height = stdout_height;

        let stdout_shown = match stdout_lock {
            Ok(stdout) => stdout
                .iter()
                .skip(self.stdout_scroll)
                .take(stdout_height)
                .rev()
                .map(|v| v.as_str())
                .collect::<String>(),
            Err(_) => String::new(),
        };

        let stdout_paragraph = Paragraph::new(stdout_shown).block(stdout_paragraph_block);

        frame.render_widget(stdout_paragraph, stdout_top);

        // Binds panel
        let binds_block = Block::bordered();

        let binds =
            Paragraph::new(" [J] scroll down │ [K] scroll up │ [i]nject │ [l]aunch │ [k]ill")
                .block(binds_block);

        frame.render_widget(binds, binds_bottom);

        // Info
        let info_block = Block::bordered().title("Info");

        let pid_text = if let Some(pid) = self.pid {
            pid.to_string()
        } else {
            "N/A".to_string()
        };

        let info_paragraph =
            Paragraph::new(format!("PID:\n{}\n\n{}", pid_text, self.injection_method))
                .block(info_block);

        frame.render_widget(info_paragraph, right);
    }

    fn handle_events_overview(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('J') => self.stdout_scroll = self.stdout_scroll.saturating_sub(1),
            KeyCode::Char('K') => {
                if self.stdout.lock().expect("Stdout mutex poisoned").len() - self.stdout_scroll
                    > self.stdout_paragraph_height
                {
                    self.stdout_scroll = self.stdout_scroll.saturating_add(1);
                }
            }
            KeyCode::Char('i') => {
                self.state = State::PidTextInput(TextInput::new("PID:".to_string()))
            }
            KeyCode::Char('l') => {
                self.state = State::LaunchTextInput(TextInput::new("Executable path:".to_string()))
            }
            KeyCode::Char('k') if let Some(pid) = self.pid.take() => {
                self.injection_method = InjectionStatus::NotInjected;
                self.stdout = Arc::new(Mutex::new(VecDeque::new()));

                let mut sys = System::new();

                sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

                if let Some(p) = sys.process(Pid::from_u32(pid)) {
                    p.kill();
                }
            }
            _ => (),
        }
    }

    fn construct_script(&mut self, frame: &mut Frame, area: Rect) {
        let [content_top, binds_bottom] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(area);

        let [content_left, content_right] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(content_top);

        let [injected_space, managed_space] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(content_left);

        // Remote & local scripts
        let remote_title = self.scripts_mgr.get_remote_title();

        let remote_block = Block::bordered().title(remote_title);

        let remote_highlight_style = if !self.scripts_mgr.local_selected {
            Style::default()
                .fg(ratatui::style::Color::Black)
                .bg(ratatui::style::Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let remote_list = List::new(
            self.scripts_mgr
                .remote_scripts
                .iter()
                .map(|(s, _)| format!("{:?}", s)),
        )
        .highlight_style(remote_highlight_style)
        .block(remote_block);

        frame.render_stateful_widget(
            remote_list,
            injected_space,
            &mut self.scripts_mgr.remote_scripts_state,
        );

        let local_title = self.scripts_mgr.get_local_title();

        let local_block = Block::bordered().title(local_title);

        let local_highlight_style = if self.scripts_mgr.local_selected {
            Style::default()
                .fg(ratatui::style::Color::Black)
                .bg(ratatui::style::Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let local_list = List::new(
            self.scripts_mgr
                .local_scripts
                .iter()
                .map(|(s, _)| s.clone()),
        )
        .highlight_style(local_highlight_style)
        .block(local_block);

        frame.render_stateful_widget(
            local_list,
            managed_space,
            &mut self.scripts_mgr.local_scripts_state,
        );

        // Script preview
        let preview_block = Block::bordered().title(if let State::ScriptPreview = self.state {
            "* Preview"
        } else {
            "Preview"
        });

        let preview_area = preview_block.inner(content_right);

        frame.render_widget(preview_block, content_right);

        self.scripts_mgr.preview.render(frame, preview_area);

        // Binds
        let binds = if let State::ScriptPreview = self.state {
            " EDITOR"
        } else {
            self.scripts_mgr.get_binds()
        };

        let binds_paragraph = Paragraph::new(binds).block(Block::bordered());

        frame.render_widget(binds_paragraph, binds_bottom);
    }

    fn handle_events_scripts(&mut self, code: KeyCode) {
        match code {
            // Generic
            KeyCode::Tab => {
                if let Some(v) = self.scripts_mgr.get_state().selected() {
                    self.scripts_mgr
                        .set_src_at(v, self.scripts_mgr.preview.get_text());
                }

                self.scripts_mgr.toggle_active();

                let new_text = match self.scripts_mgr.get_state().selected() {
                    Some(v) => self.scripts_mgr.get_src_at(v).unwrap_or_default(),
                    None => String::default(),
                };

                self.scripts_mgr.preview.set_text(new_text);
            }

            KeyCode::Enter
                if let Some(index) = self.scripts_mgr.get_state().selected()
                    && index < self.scripts_mgr.get_sel_len() =>
            {
                self.scripts_mgr.update_preview_perms();
                self.state = State::ScriptPreview;
            }
            KeyCode::Char('j') => {
                if let Some(index) = self.scripts_mgr.get_state().selected() {
                    self.scripts_mgr
                        .set_src_at(index, self.scripts_mgr.preview.get_text());
                }

                self.scripts_mgr.get_state().select_next();

                if let Some(index) = self.scripts_mgr.get_state().selected()
                    && let Some(src) = self.scripts_mgr.get_src_at(index)
                {
                    self.scripts_mgr.preview.set_text(src);
                }
            }
            KeyCode::Char('k') => {
                if let Some(index) = self.scripts_mgr.get_state().selected() {
                    self.scripts_mgr
                        .set_src_at(index, self.scripts_mgr.preview.get_text());
                }

                self.scripts_mgr.get_state().select_previous();

                if let Some(index) = self.scripts_mgr.get_state().selected()
                    && let Some(src) = self.scripts_mgr.get_src_at(index)
                {
                    self.scripts_mgr.preview.set_text(src);
                }
            }

            // Remote scripts bindings
            a if !self.scripts_mgr.local_selected => match a {
                KeyCode::Char('u') => {
                    if let Some(index) = self.scripts_mgr.get_state().selected()
                        && let Some(ref socket) = self.socket
                        && let Some((id, _)) = self.scripts_mgr.remote_scripts.try_remove(index)
                    {
                        if let Err(e) = unload_script(socket, id) {
                            self.state = State::DisplayError(format!(
                                "Unable to unload script, error: {}",
                                e
                            ));
                        }

                        self.clamp_selection();

                        let new_text = match self.scripts_mgr.get_state().selected() {
                            Some(v) => self.scripts_mgr.get_src_at(v).unwrap_or_default(),
                            None => String::default(),
                        };

                        self.scripts_mgr.preview.set_text(new_text);
                    }
                }
                _ => (),
            },

            // Local scripts bindings
            a if self.scripts_mgr.local_selected => match a {
                KeyCode::Char('a') => {
                    self.state =
                        State::AddScriptTextInput(TextInput::new("Script path:".to_string()))
                }

                KeyCode::Char('i') => {
                    if let Some(index) = self.scripts_mgr.local_scripts_state.selected()
                        && let Some(ref socket) = self.socket
                        && let Some((_, source)) = self.scripts_mgr.local_scripts.try_remove(index)
                    {
                        if let Err(e) = load_script(socket, source) {
                            self.state =
                                State::DisplayError(format!("Unable to load script, error: {}", e));
                        }

                        match remote_scripts(socket) {
                            Ok(scripts) => self.scripts_mgr.remote_scripts = scripts,
                            Err(e) => {
                                self.state = State::DisplayError(format!(
                                    "Unable to fetch remote scripts, error: {}",
                                    e
                                ))
                            }
                        }

                        self.clamp_selection();

                        let new_text = match self.scripts_mgr.get_state().selected() {
                            Some(v) => self.scripts_mgr.get_src_at(v).unwrap_or_default(),
                            None => String::default(),
                        };

                        self.scripts_mgr.preview.set_text(new_text);
                    }
                }
                _ => (),
            },

            _ => (),
        }
    }

    fn clamp_selection(&mut self) {
        let len = self.scripts_mgr.get_sel_len();
        let state = self.scripts_mgr.get_state();

        match state.selected() {
            Some(_) if len == 0 => state.select(None),
            Some(i) if i >= len => state.select(Some(len - 1)),
            _ => (),
        }
    }
}
