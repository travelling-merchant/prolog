use ratatui::{prelude::*, widgets::Paragraph};

use crate::{InputCommands, handle_events};
pub fn render_math(frame: &mut Frame) {
    for pixel in 0..10 {
        let area = Rect::new(0, pixel, frame.area().width, 1);
        frame.render_widget(Paragraph::new("fuck you!!!"), area);
    }
    dbg!("not implemented");
}
