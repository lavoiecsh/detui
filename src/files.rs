use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders};
use crate::component::Component;

pub struct Files {}

impl Files {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Files {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::default().borders(Borders::TOP).title("Files"), area);
    }
}
