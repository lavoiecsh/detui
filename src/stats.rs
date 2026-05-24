use crate::model::DeTuiModel;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Style, Widget};

#[derive(Debug)]
pub struct StatsWidget<'a> {
    model: &'a DeTuiModel,
}

impl<'a> StatsWidget<'a> {
    pub fn new(model: &'a DeTuiModel) -> Self {
        StatsWidget { model }
    }
    
    pub fn lines(&self) -> u16 { 4 }
}

impl Widget for StatsWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        buf.set_string(area.left(), area.top(), format!("Counter: {}", self.model.counter()), Style::default());
    }
}