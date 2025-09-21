use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_math(frame: &mut Frame) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let question = Paragraph::new("What is anything?")
        .block(Block::default().title(" Question ").borders(Borders::ALL));
    frame.render_widget(question, chunks[0]);
    let answer_collection = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let row_one = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(answer_collection[0]);

    let row_two = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(answer_collection[1]);

    let answer_option_one =
        Paragraph::new("B").block(Block::default().title(" Answer ").borders(Borders::ALL));
    let answer_option_two =
        Paragraph::new("A").block(Block::default().title(" Answer ").borders(Borders::ALL));
    let answer_option_three =
        Paragraph::new("C").block(Block::default().title(" Answer ").borders(Borders::ALL));
    let answer_option_four =
        Paragraph::new("D").block(Block::default().title(" Answer ").borders(Borders::ALL));

    frame.render_widget(answer_option_one, row_one[0]);
    frame.render_widget(answer_option_two, row_one[1]);
    frame.render_widget(answer_option_three, row_two[0]);
    frame.render_widget(answer_option_four, row_two[1]);
}
