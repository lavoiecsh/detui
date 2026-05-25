mod config;
mod app;
mod stats;
mod model;
mod files;
mod shell;

use crate::app::DeTuiApp;
use crate::config::Configuration;
use std::io;
use ratatui::crossterm::terminal;

fn main() -> io::Result<()> {
    let configuration = Configuration::default();
    let mut terminal = ratatui::init();
    terminal::enable_raw_mode()?;
    let result = DeTuiApp::run(configuration, &mut terminal);
    terminal::disable_raw_mode()?;
    ratatui::restore();
    result
}
