use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Frame, text::Text};
#[derive(PartialEq)]
enum InputCommands {
    ContinueAction,
    Quit,
}
fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}
fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| draw_title_screen(frame))?;
        let input_result = handle_events();
        match input_result {
            Ok(result_here) => {
                if result_here == InputCommands::Quit {
                    break Ok(());
                }
                if result_here == InputCommands::ContinueAction {
                    terminal.draw(|frame| n(frame))?;
                }
            }
            Err(e) => {
                break Err(e);
            }
        }
    }
}
fn handle_events() -> std::io::Result<InputCommands> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(InputCommands::Quit),
            _ => {}
        },
        _ => {}
    }
    Ok(InputCommands::ContinueAction)
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
