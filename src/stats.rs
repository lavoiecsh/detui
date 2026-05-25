use crate::model::DeTuiModel;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Style, Widget};

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
        buf.set_string(area.left(), area.top() + 0, "statistic 0", Style::default());
        buf.set_string(area.left(), area.top() + 1, "statistic 1", Style::default());
        buf.set_string(area.left(), area.top() + 2, "statistic 2", Style::default());
        buf.set_string(area.left(), area.top() + 3, "statistic 3", Style::default());
    }
}