use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use crate::config::Configuration;
use crate::shell::Shell;

pub struct DeTuiModel {
    pub state: DeTuiState,
    configuration: Configuration,
    pub shell: Shell,
}

#[derive(Debug, Clone, Copy)]
pub enum DeTuiState {
    Exiting,
    Terminal,
    Control,
    Files,
}

impl DeTuiModel {
    pub fn new(configuration: Configuration, shell: Shell) -> Self {
        Self {
            state: DeTuiState::Terminal,
            configuration,
            shell,
        }
    }

    pub fn is_running(&self) -> bool {
        !matches!(self.state, DeTuiState::Exiting)
    }

    pub fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        self.state = match (self.state, code, modifiers) {
            (_, KeyCode::Char('z'), KeyModifiers::CONTROL) => DeTuiState::Control,
            (DeTuiState::Control, KeyCode::Char('q'), _) => DeTuiState::Exiting,
            (DeTuiState::Control, KeyCode::Char('t'), _) => DeTuiState::Terminal,
            (DeTuiState::Control, KeyCode::Char('f'), _) => DeTuiState::Files,
            (DeTuiState::Terminal, _, _) => {
                self.shell.handle_key(code, modifiers);
                DeTuiState::Terminal
            }
            _ => self.state,
        };
    }
}
