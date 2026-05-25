use crate::config::Configuration;
use crate::shell::Shell;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::prelude::Rect;
use std::path::PathBuf;
use std::process::Command;
use std::{env, io};

pub struct DeTuiModel {
    pub state: DeTuiState,
    pub configuration: Configuration,
    pub shell: Shell,
    pub base_directory: PathBuf,
    pub base_repository: String,
    pub base_branch: String,
}

#[derive(Debug, Clone, Copy)]
pub enum DeTuiState {
    Exiting,
    Terminal,
    Control,
    Files,
}

impl DeTuiModel {
    pub fn new(configuration: Configuration, area: Rect) -> io::Result<Self> {
        let base_directory = env::current_dir()?;
        let base_repository = Command::new("gh")
            .args(["repo", "view", "--json", "name", "-q", ".name"])
            .output()?;
        let base_branch = Command::new("git")
            .args(["branch", "--show-current"])
            .output()?;

        Ok(Self {
            state: DeTuiState::Terminal,
            configuration,
            // todo! don't specify size here
            shell: Shell::new(area.height, area.width * 3 / 4, &base_directory),
            base_directory,
            base_repository: String::from_utf8_lossy(&base_repository.stdout).to_string(),
            base_branch: String::from_utf8_lossy(&base_branch.stdout).to_string(),
        })
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
