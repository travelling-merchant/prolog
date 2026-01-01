use crossterm::style::Color;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::MathState;
use crate::thinking_in_boxes::AppData;

pub fn render_math(frame: &mut Frame, app_data: &AppData, state: &MathState) {
    let area = frame.area();
    //let test = &app_data.get_random_data_by_topic().text;
    let question_list = &app_data.get_data_list_for_topic(&"Math");
    let current_question = &app_data.get_random_data_by_topic(question_list);
    let test = current_question.question.text.to_string();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let question = Paragraph::new(current_question.question.text.to_string())
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

    //let selected_style = Style::default().bg(Color::Blue).fg(Color::White);
    let normal_style = Style::default();
    let answer_option_one = Paragraph::new("A").block(
        Block::default()
            .title(test.to_string())
            .borders(Borders::ALL),
    );
    let answer_option_two = Paragraph::new("b").block(
        Block::default()
            .title(" Answer ")
            .borders(Borders::ALL)
            .bg(ratatui::prelude::Color::Red),
    );
    let answer_option_three =
        Paragraph::new("C").block(Block::default().title(" Answer ").borders(Borders::ALL));
    let answer_option_four =
        Paragraph::new("D").block(Block::default().title(" Answer ").borders(Borders::ALL));

    frame.render_widget(answer_option_one, row_one[0]);
    frame.render_widget(answer_option_two, row_one[1]);
    frame.render_widget(answer_option_three, row_two[0]);
    frame.render_widget(answer_option_four, row_two[1]);
}
