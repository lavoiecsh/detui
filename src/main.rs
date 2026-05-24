mod config;
mod app;
mod stats;
mod model;
mod files;

use crate::config::Configuration;
use std::io;
use crate::app::DeTuiApp;

fn main() -> io::Result<()> {
    let config = Configuration::default();
    let mut app = DeTuiApp::new(config);
    ratatui::run(|t| app.run(t))
}
