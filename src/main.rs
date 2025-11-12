use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{Frame, text::Text};

use crate::thinking_in_boxes::AppData;
mod exams;
mod thinking_in_boxes;

#[derive(PartialEq)]
enum InputCommands {
    ContinueAction,
    Quit,
    StartMathExam,
}

#[derive(PartialEq)]
enum OverallState {
    TitleScreen,
    MathExam,
}
fn main() -> std::io::Result<()> {
    let exam_data =
        thinking_in_boxes::everything_is_an_abstraction().expect("Failed to load data on startup");
    let full_data = AppData {
        exam_data: exam_data,
    };
    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &full_data);
    ratatui::restore();
    result
}
fn run(terminal: &mut ratatui::DefaultTerminal, app_data: &AppData) -> std::io::Result<()> {
    let mut current_state_ffs: OverallState = OverallState::TitleScreen;

    loop {
        terminal.draw(|frame| draw_title_screen(frame))?;
        terminal.draw(|frame| match current_state_ffs {
            OverallState::TitleScreen => draw_title_screen(frame),
            OverallState::MathExam => exams::math::render_math(frame, app_data),
        })?;

        let input_result = handle_events();
        match input_result {
            Ok(result_here) => match result_here {
                InputCommands::Quit => break Ok(()),
                InputCommands::ContinueAction => {
                    terminal.draw(|frame| n(frame))?;
                }
                InputCommands::StartMathExam => {
                    current_state_ffs = OverallState::MathExam;
                }
            },
            Err(e) => {
                break Err(e);
            }
        }
    }
}

fn handle_events() -> std::io::Result<InputCommands> {
    match event::read()? {
        Event::Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) => match (code, modifiers) {
            (KeyCode::Char('c') | KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                Ok(InputCommands::Quit)
            }
            _ => Ok(InputCommands::StartMathExam),
        },
        _ => Ok(InputCommands::ContinueAction),
    }
}

const ASCII_TITLE_SCREEN: &str = r#"
                .                                            .
     *   .                  .              .        .   *          .
  .         .                     .       .           .      .        .
        o                             .                   .
         .              . PROLOG                  .           .
          0     .
                 .          .   Press any button to continue            
 .          \          .                         .
      .      \   ,
   .          o     .                 .                   .            .
     .         \                 ,             .                .
               #\##\#      .                              .        .
             #  #O##\###                .                        .
   .        #*#  #\##\###                       .                     ,
        .   ##*#  #\##\##               .                     .
      .      ##*#  #o##\#         .                             ,       .
          .     *#  #\#     .                    .             .          ,
                      \          .                         .
____^/\___^--____/\____O______________/\/\---/\___________---______________
   /\^   ^  ^    ^                  ^^ ^  '\ ^          ^       ---
         --           -            --  -      -         ---  __       ^
   --  __                      ___--  ^  ^                         --  __

"#;

fn draw_title_screen(frame: &mut Frame) {
    let text = Text::raw(ASCII_TITLE_SCREEN);
    frame.render_widget(text, frame.area());
}
fn n(frame: &mut Frame) {
    let text = Text::raw("n");
    frame.render_widget(text, frame.area());
}
