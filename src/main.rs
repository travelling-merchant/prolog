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
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
}

#[derive(PartialEq)]
enum OverallState {
    TitleScreen,
    MathExam(MathState),
}
#[derive(PartialEq, Default)]
pub struct MathState {
    pub selected: usize,
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
        terminal.draw(|frame| match &current_state_ffs {
            OverallState::TitleScreen => draw_title_screen(frame),
            OverallState::MathExam(state) => exams::math::render_math(frame, app_data, &state),
        })?;

        let input_result = handle_events()?;
        current_state_ffs = match current_state_ffs {
            OverallState::TitleScreen => match input_result {
                InputCommands::Quit => break Ok(()),
                InputCommands::StartMathExam => OverallState::MathExam(MathState::default()),
                _ => OverallState::TitleScreen,
            },

            OverallState::MathExam(mut state) => match input_result {
                InputCommands::Quit => break Ok(()),
                InputCommands::MoveUp => {
                    if state.selected >= 2 {
                        state.selected -= 2;
                    }
                    OverallState::MathExam(state)
                }
                InputCommands::MoveDown => {
                    if state.selected < 2 {
                        state.selected += 2;
                    }
                    OverallState::MathExam(state)
                }
                InputCommands::MoveLeft => {
                    if state.selected % 2 == 1 {
                        state.selected -= 1;
                    }
                    OverallState::MathExam(state)
                }
                InputCommands::MoveRight => {
                    if state.selected % 2 == 0 && state.selected < 3 {
                        state.selected += 1;
                    }
                    OverallState::MathExam(state)
                }

                _ => OverallState::MathExam(state),
            },
        };
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
            (KeyCode::Char('k') | KeyCode::Char('w'), KeyModifiers::NONE) => {
                Ok(InputCommands::MoveUp)
            }
            (KeyCode::Char('h') | KeyCode::Char('a'), KeyModifiers::NONE) => {
                Ok(InputCommands::MoveLeft)
            }
            (KeyCode::Char('l') | KeyCode::Char('d'), KeyModifiers::NONE) => {
                Ok(InputCommands::MoveRight)
            }
            (KeyCode::Char('j') | KeyCode::Char('s'), KeyModifiers::NONE) => {
                Ok(InputCommands::MoveDown)
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
