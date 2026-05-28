mod agent;
mod app;
mod component;
mod config;
mod files;
mod shell;
mod stats;

use crate::app::DeTuiApp;
use crate::component::{Component, DeTuiEvent};
use crate::config::Configuration;
use ratatui::crossterm::event::{Event, KeyEventKind};
use ratatui::crossterm::{event, terminal};
use ratatui::DefaultTerminal;
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let configuration = Configuration::read()?;
    let mut app = DeTuiApp::new(configuration)?;
    let mut terminal = ratatui::init();
    terminal::enable_raw_mode()?;
    let result = run(&mut app, &mut terminal);
    terminal::disable_raw_mode()?;
    ratatui::restore();
    result
}

fn run(app: &mut DeTuiApp, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
    terminal.draw(|f| app.render(f, f.area()))?;
    while app.is_running() {
        terminal.draw(|f| app.render(f, f.area()))?;
        app.handle_event(poll_event()?);
    }
    Ok(())
}

fn poll_event() -> Result<DeTuiEvent, Box<dyn Error>> {
    loop {
        let mut count = 0;
        while !event::poll(Duration::from_millis(10))? {
            count += 1;
            if count == 10 {
                return Ok(DeTuiEvent::Refresh);
            }
        }

        if let Event::Key(key_event) = event::read()?
            && key_event.kind == KeyEventKind::Press
        {
            return Ok(DeTuiEvent::KeyPress(key_event));
        }
    }
}
