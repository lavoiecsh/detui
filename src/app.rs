use crate::agent::Agent;
use crate::component::{Component, DeTuiEvent};
use crate::config::Configuration;
use crate::files::Files;
use crate::shell::Shell;
use crate::stats::Stats;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;
use std::error::Error;

pub struct DeTuiApp<'a> {
    configuration: Configuration,
    state: DeTuiAppState,
    main_state: DeTuiMainState,
    stats: Stats,
    files: Files,
    shell: Shell,
    agent: Agent<'a>,
}

enum DeTuiAppState {
    Exiting,
    Control,
    Files,
    Shell,
    Agent,
}

enum DeTuiMainState {
    Shell,
    Agent,
}

impl DeTuiAppState {
    fn is_running(&self) -> bool {
        !matches!(self, DeTuiAppState::Exiting)
    }
}

impl DeTuiApp<'_> {
    pub fn new(configuration: Configuration) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            state: DeTuiAppState::Shell,
            main_state: DeTuiMainState::Shell,
            stats: Stats::new()?,
            files: Files::new(),
            shell: Shell::new(&configuration),
            agent: Agent::new(),
            configuration,
        })
    }

    pub fn is_running(&self) -> bool {
        self.state.is_running()
    }
}

impl Component for DeTuiApp<'_> {
    fn handle_event(&mut self, event: DeTuiEvent) -> Option<DeTuiEvent> {
        match event {
            DeTuiEvent::Refresh => None,
            DeTuiEvent::KeyPress(key_event) => {
                match (&self.state, key_event.code, key_event.modifiers) {
                    (_, KeyCode::Char('z'), KeyModifiers::CONTROL) => {
                        self.state = DeTuiAppState::Control;
                        None
                    }
                    (DeTuiAppState::Control, KeyCode::Char('q'), _) => {
                        self.state = DeTuiAppState::Exiting;
                        None
                    }
                    (DeTuiAppState::Control, KeyCode::Char('t'), _) => {
                        self.state = DeTuiAppState::Shell;
                        self.main_state = DeTuiMainState::Shell;
                        None
                    }
                    (DeTuiAppState::Control, KeyCode::Char('a'), _) => {
                        self.state = DeTuiAppState::Agent;
                        self.main_state = DeTuiMainState::Agent;
                        None
                    }
                    (DeTuiAppState::Control, KeyCode::Char('f'), _) => {
                        self.state = DeTuiAppState::Files;
                        None
                    }
                    _ => None,
                }
            }
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let [sidebar_area, main_area] = area.layout(&Layout::horizontal([
            Constraint::Ratio(1, 4),
            Constraint::Fill(1),
        ]));

        let block = Block::default().borders(Borders::RIGHT);
        frame.render_widget(&block, sidebar_area);
        let [stats_area, files_area] = block.inner(sidebar_area).layout(&Layout::vertical([
            Constraint::Length(self.stats.lines()),
            Constraint::Fill(1),
        ]));
        self.stats.render(frame, stats_area);
        self.files.render(frame, files_area);

        match self.main_state {
            DeTuiMainState::Shell => self.shell.render(frame, main_area),
            DeTuiMainState::Agent => self.agent.render(frame, main_area),
        }
    }
}
