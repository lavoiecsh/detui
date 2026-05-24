use crate::config::Configuration;
use crate::files::FilesWidget;
use crate::model::{DeTuiModel, DeTuiState};
use crate::stats::StatsWidget;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use std::time::Duration;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use crate::model::DeTuiMessage;

#[derive(Debug)]
pub struct DeTuiApp {
    config: Configuration,
    model: DeTuiModel,
}

impl DeTuiApp {
    pub fn new(config: Configuration) -> Self {
        Self {
            model: DeTuiModel::new(),
            config,
        }
    }
}

impl DeTuiApp {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !matches!(self.model.state, DeTuiState::Exiting) {
            terminal.draw(|f| self.draw(f))?;
            if let Some(message) = self.handle_events()? {
                if let Some(model) = self.model.handle_message(message) {
                    self.model = model;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let stats = StatsWidget::new(&self.model);
        let files = FilesWidget::new(&self.model);
        let main = Paragraph::new("");

        let [sidebar_area, main_outer] = frame.area().layout(&Layout::horizontal([Constraint::Ratio(1, 4), Constraint::Fill(1)]));
        let [stats_outer, files_outer] = sidebar_area.layout(&Layout::vertical([Constraint::Length(stats.lines()), Constraint::Fill(1)]));

        let stats_block = Block::default().borders(Borders::TOP).title("Stats");
        let stats_inner = stats_block.inner(stats_outer);
        frame.render_widget(stats_block, stats_outer);
        frame.render_widget(stats, stats_inner);

        let files_block = Block::default().borders(Borders::TOP).title("Files");
        let files_inner = files_block.inner(files_outer);
        frame.render_widget(files_block, files_outer);
        frame.render_widget(files, files_inner);

        let main_block = Block::default().borders(Borders::LEFT).title("Main");
        let main_inner = main_block.inner(main_outer);
        frame.render_widget(main_block, main_outer);
        frame.render_widget(main, main_inner);
    }

    fn handle_events(&mut self) -> io::Result<Option<DeTuiMessage>> {
        let poll_result = event::poll(Duration::from_millis(100))?;
        if !poll_result { return Ok(None); }

        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => Ok(Some(DeTuiMessage::Exit)),
                    KeyCode::Char('i') | KeyCode::Up => Ok(Some(DeTuiMessage::Increment)),
                    KeyCode::Char('d') | KeyCode::Down => Ok(Some(DeTuiMessage::Decrement)),
                    _ => Ok(None),
                }
            },
            _ => Ok(None),
        }
    }
}
