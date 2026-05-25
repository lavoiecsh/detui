use crate::model::DeTuiModel;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;

pub struct FilesWidget<'a> {
    model: &'a DeTuiModel,
}

impl<'a> FilesWidget<'a> {
    pub fn new(model: &'a DeTuiModel) -> Self {
        FilesWidget { model }
    }
}

impl Widget for FilesWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}
