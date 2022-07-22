use crate::app::AppCtx;
use crate::render::Render;
use std::time::{Duration, Instant};
use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Paragraph, Widget};

pub struct MessageBar {
    content: String,
    time: Instant,
    delay: Duration,
}

impl Render for MessageBar {
    fn name(&self) -> String {
        "message".to_string()
    }

    fn render(&mut self, ctx: AppCtx, buf: &mut Buffer, area: Rect) {
        if !self.should_render() {
            return;
        }

        Paragraph::new(format!("text message: {}", self.content))
            .alignment(Alignment::Center)
            .style(Style::default().bg(Color::Red))
            .render(area, buf);
    }
}

impl MessageBar {
    pub fn new(content: String, delay: Duration) -> Self {
        Self {
            content,
            time: Instant::now(),
            delay,
        }
    }

    /// should render message bar
    pub fn should_render(&self) -> bool {
        if self.content.len() == 0 {
            return false;
        }

        Instant::now() - self.time < self.delay
    }
}

impl Default for MessageBar {
    fn default() -> Self {
        MessageBar::from("")
    }
}

impl From<&str> for MessageBar {
    fn from(message: &str) -> Self {
        MessageBar::from(message.to_string())
    }
}

/// default
impl From<String> for MessageBar {
    fn from(message: String) -> Self {
        Self {
            content: message,
            time: Instant::now(),
            delay: Duration::new(5, 0),
        }
    }
}
