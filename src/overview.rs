use crate::app::Tab;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{Paragraph, Widget, WidgetRef},
};

pub struct Overview;

impl Tab for Overview {
    fn title(&self) -> String {
        "Overview".to_string()
    }

    fn color(&self) -> Color {
        Color::Red
    }
}

impl WidgetRef for Overview {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Overview").render(area, buf);
    }
}
