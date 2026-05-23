use ratatui::prelude::{Constraint, Direction, Layout, Widget};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

#[derive(Debug, Default)]
struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 4), Constraint::Fill(1)])
            .split(frame.area());

        frame.render_widget(
            Paragraph::new("SideBar").block(Block::new().borders(Borders::ALL)),
            layout[0],
        );
        frame.render_widget(
            Paragraph::new("Main"),
            layout[1],
        );
    }

    fn handle_events(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|t| App::default().run(t))
}
