use crate::component::Component;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::Frame;
use std::error::Error;
use std::process::Command;
use ratatui::widgets::{Block, Borders};

pub struct Stats {
    repository: String,
    branch: String,
}

impl<'a> Stats {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let repository = String::from_utf8(
            Command::new("gh")
                .args(["repo", "view", "--json", "name", "-q", ".name"])
                .output()?
                .stdout,
        )?;
        let branch = String::from_utf8(
            Command::new("git")
                .args(["branch", "--show-current"])
                .output()?
                .stdout
        )?;
        Ok(Stats {
            repository,
            branch,
        })
    }

    pub fn lines(&self) -> u16 {
        2 + 1
    }
}

impl Component for Stats {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::TOP).title("Stats");
        frame.render_widget(&block, area);
        let buf = frame.buffer_mut();

        buf.set_string(
            block.inner(area).left(),
            block.inner(area).top() + 0,
            format!("repo  : {}", self.repository),
            Style::default(),
        );
        buf.set_string(
            block.inner(area).left(),
            block.inner(area).top() + 1,
            format!("branch: {}", self.branch),
            Style::default(),
        );
    }
}
