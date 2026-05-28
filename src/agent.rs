use crate::component::Component;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::Frame;
use ratatui_textarea::TextArea;

pub struct Agent<'a> {
    prompt: TextArea<'a>,
}

impl<'a> Agent<'a> {
    pub fn new() -> Self {
        Self {
            prompt: TextArea::default(),
        }
    }
}

impl Component for Agent<'_> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.buffer_mut().set_string(area.left(), area.top(), "agent", Style::default());
        frame.render_widget(&self.prompt, area);
    }
}