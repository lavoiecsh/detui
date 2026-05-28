use ratatui::crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use ratatui::Frame;

pub trait Component {
    fn handle_event(&mut self, event: DeTuiEvent) -> Option<DeTuiEvent> {
        None
    }

    fn render(&self, frame: &mut Frame, area: Rect) {}
}

pub enum DeTuiEvent {
    Refresh,
    KeyPress(KeyEvent),
}
