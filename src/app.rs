use crate::config::Configuration;
use crate::files::FilesWidget;
use crate::model::DeTuiModel;
use crate::stats::StatsWidget;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Borders};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use std::time::Duration;
use tui_term::widget::PseudoTerminal;

pub struct DeTuiApp {
    model: DeTuiModel,
}

impl DeTuiApp {
    pub fn run(configuration: Configuration, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let main_area = terminal.get_frame().area();
        let model = DeTuiModel::new(configuration, main_area)?;
        let mut app = DeTuiApp { model };
        while app.model.is_running() {
            terminal.draw(|f| app.draw(f))?;
            app.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let stats = StatsWidget::new(&self.model);
        let files = FilesWidget::new(&self.model);
        let parser = self.model.shell.parser.read().unwrap();
        let main = PseudoTerminal::new(parser.screen());

        let [sidebar_area, main_outer] = frame.area().layout(&Layout::horizontal([
            Constraint::Ratio(1, 4),
            Constraint::Fill(1),
        ]));
        let [stats_outer, files_outer] = sidebar_area.layout(&Layout::vertical([
            Constraint::Length(stats.lines() + 1),
            Constraint::Fill(1),
        ]));

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

    fn handle_events(&mut self) -> io::Result<()> {
        let poll_result = event::poll(Duration::from_millis(10))?;
        if !poll_result {
            return Ok(());
        }

        if let Event::Key(key_event) = event::read()?
            && key_event.kind == KeyEventKind::Press
        {
            self.model.handle_key(key_event.code, key_event.modifiers);
        }
        Ok(())
    }
}
