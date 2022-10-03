use miro_tui::structs::Position;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use serde_json::{json, Value};
use std::env;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board_name = "RustyBrown";
    let board_id = "uXjVPRi1YRA="; //  "uXjVPRG-bRg=";
                                   // use std::{thread, time};

    // let ten_millis = time::Duration::from_millis(1000);
    // let now = time::Instant::now();

    // thread::sleep(ten_millis);
    // create_board(board_name)?;
    let mut prev_note = miro_tui::create_sticky_note(
        &board_id,
        &String::from("0"),
        Position { x: 0.0, y: 600.0 },
        "square",
    )?;
    for idx in 1..5 {
        let next_note = miro_tui::create_sticky_note(
            &board_id,
            &format!("{}", idx),
            Position {
                x: idx as f32 * 260.0,
                y: idx as f32 * 250.0,
            },
            "square",
        )?;
        println!("{}", next_note.style.fill_color);
        let _ = miro_tui::create_connector(&board_id, &prev_note.id, &next_note.id);
        prev_note = next_note;
    }
    Ok(())
}
