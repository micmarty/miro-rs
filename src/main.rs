use miro_rs::structs::Position;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use serde_json::{json, Value};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board_name = "RustyBrown";
    let board_id = "uXjVPRi1YRA=";
    // use std::{thread, time};
    // let ten_millis = time::Duration::from_millis(1000);
    // let now = time::Instant::now();

    // thread::sleep(ten_millis);
    // create_board(board_name)?;
    let mut prev_note = miro_rs::create_sticky_note(
        &board_id,
        &String::from("0"),
        Position { x: 0.0, y: 600.0 },
        "square",
    )
    .expect("dasd");
    for idx in 1..5 {
        let next_note = miro_rs::create_sticky_note(
            &board_id,
            &format!("{}", idx),
            Position {
                x: idx as f32 * 260.0,
                y: idx as f32 * 250.0,
            },
            "square",
        );
        if let Some(next_note) = next_note {
            println!("{}", next_note.style.fill_color);
            let _ = miro_rs::create_connector(&board_id, &prev_note.id, &next_note.id);
            prev_note = next_note;
        }
    }
    Ok(())
}

// struct + trait
// https://github.com/LittleBigBug/LakituLib/blob/6627f1bfa3d2d3fa60aabfc1abef7c6828a336a7/src/duppy/mod.rs

// match resp.status() {
//     StatusCode::OK => println!("success!"),
//     StatusCode::PAYLOAD_TOO_LARGE => {
//         println!("Request payload is too large!");
//     }
//     s => println!("Received response status: {:?}", s),
// };

// println!("{:#?}", resp);
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use std::{error::Error, io};
// use tui::{
//     backend::{Backend, CrosstermBackend},
//     layout::{Constraint, Direction, Layout},
//     style::{Color, Modifier, Style},
//     text::{Span, Spans, Text},
//     widgets::{Block, Borders, List, ListItem, Paragraph},
//     Frame, Terminal,
// };
// use unicode_width::UnicodeWidthStr;

// enum InputMode {
//     Normal,
//     Editing,
// }

// /// App holds the state of the application
// struct App {
//     /// Current value of the input box
//     input: String,
//     /// Current input mode
//     input_mode: InputMode,
//     /// History of recorded messages
//     messages: Vec<String>,
// }

// impl Default for App {
//     fn default() -> App {
//         App {
//             input: String::new(),
//             input_mode: InputMode::Normal,
//             messages: Vec::new(),
//         }
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     // create app and run it
//     let app = App::default();
//     let res = run_app(&mut terminal, app).await;

//     // restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;

//     if let Err(err) = res {
//         println!("{:?}", err)
//     }

//     Ok(())
// }

// async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
//     loop {
//         terminal.draw(|f| ui(f, &app))?;

//         if let Event::Key(key) = event::read()? {
//             match app.input_mode {
//                 InputMode::Normal => match key.code {
//                     KeyCode::Char('e') => {
//                         app.input_mode = InputMode::Editing;
//                     }
//                     KeyCode::Char('q') => {
//                         return Ok(());
//                     }
//                     _ => {}
//                 },
//                 InputMode::Editing => match key.code {
//                     KeyCode::Enter => {
//                         let text = app.input.drain(..).collect();
//                         let board_id = "uXjVPRi1YRA="; //  "uXjVPRG-bRg=";
//                         let prev_note =
//                             create_sticky_note(&board_id, &text, Point { x: 0, y: 600 }, "square")
//                                 .await
//                                 .unwrap();
//                         app.messages.push(text);
//                     }
//                     KeyCode::Char(c) => {
//                         app.input.push(c);
//                     }
//                     KeyCode::Backspace => {
//                         app.input.pop();
//                     }
//                     KeyCode::Esc => {
//                         app.input_mode = InputMode::Normal;
//                     }
//                     _ => {}
//                 },
//             }
//         }
//     }
// }

// fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(2)
//         .constraints(
//             [
//                 Constraint::Length(1),
//                 Constraint::Length(3),
//                 Constraint::Min(1),
//             ]
//             .as_ref(),
//         )
//         .split(f.size());

//     let (msg, style) = match app.input_mode {
//         InputMode::Normal => (
//             vec![
//                 Span::raw("Press "),
//                 Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to exit, "),
//                 Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to start editing."),
//             ],
//             Style::default().add_modifier(Modifier::RAPID_BLINK),
//         ),
//         InputMode::Editing => (
//             vec![
//                 Span::raw("Press "),
//                 Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to stop editing, "),
//                 Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to record the message"),
//             ],
//             Style::default(),
//         ),
//     };
//     let mut text = Text::from(Spans::from(msg));
//     text.patch_style(style);
//     let help_message = Paragraph::new(text);
//     f.render_widget(help_message, chunks[0]);

//     let input = Paragraph::new(app.input.as_ref())
//         .style(match app.input_mode {
//             InputMode::Normal => Style::default(),
//             InputMode::Editing => Style::default().fg(Color::Yellow),
//         })
//         .block(Block::default().borders(Borders::ALL).title("Input"));
//     f.render_widget(input, chunks[1]);
//     match app.input_mode {
//         InputMode::Normal =>
//             // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
//             {}

//         InputMode::Editing => {
//             // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
//             f.set_cursor(
//                 // Put cursor past the end of the input text
//                 chunks[1].x + app.input.width() as u16 + 1,
//                 // Move one line down, from the border to the input line
//                 chunks[1].y + 1,
//             )
//         }
//     }

//     let messages: Vec<ListItem> = app
//         .messages
//         .iter()
//         .enumerate()
//         .map(|(i, m)| {
//             let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
//             ListItem::new(content)
//         })
//         .collect();
//     let messages =
//         List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
//     f.render_widget(messages, chunks[2]);
// }

// fn create_board(name: &str) -> Result<Response, reqwest::Error> {
//     let url = "https://api.miro.com/v2/boards";
//     let api_token = env::var("MIRO_ACCESS_TOKEN").expect("env var not set");
//     let client = Client::new();
//     let body = json!({
//         "name": name,
//         "description": "TODO",
//         "policy": {
//             "permissionsPolicy": {
//                 "collaborationToolsStartAccess": "all_editors",
//                 "copyAccess": "anyone",
//                 "sharingAccess": "team_members_with_editing_rights"
//             },
//             "sharingPolicy": {
//                 "access": "private",
//                 "inviteToAccountAndBoardLinkAccess": "no_access",
//                 "organizationAccess": "private",
//                 "teamAccess": "private"
//             }
//         }
//     });
//     client
//         .post(url)
//         .header(ACCEPT, "application/json")
//         .header(CONTENT_TYPE, "application/json")
//         .bearer_auth(api_token)
//         .json(&body)
//         .send()
//         .await?
// }

// enum Color {
//     gray, light_yellow, yellow, orange, light_green, green, dark_green, cyan, light_pink, pink, violet, red, light_blue, blue, dark_blue, black
// }

// impl ToString for Color {
// }jj

// struct Style {
//     fill_color:
// }

// impl Default for Geometry {
//     fn default() -> Self {
//         Geometry {
//             width: 200.0,
//             height: 200.0,
//             rotation: Some(0.0),
//         }
//     }
// }
