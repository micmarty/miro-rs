use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_json::{Deserializer, Error};
use std::env;
use std::panic::PanicInfo;
pub mod structs;
// use http::StatusCode;
use structs::{
    Connector, Geometry, Position, StickyNoteCreate, StickyNoteData, StickyNoteGeometry,
    StickyNoteResponse, StickyNoteShape,
};

use crate::structs::MiroResponseError;

pub fn create_sticky_note(
    board: &str,
    text: &String,
    pos: Position,
    shape: &str,
) -> Option<StickyNoteResponse> {
    let url = format!("https://api.miro.com/v2/boards/{}/sticky_notes", board);
    let api_token = env::var("MIRO_ACCESS_TOKEN").expect("env var not set");
    let client = Client::new();

    let note = StickyNoteCreate {
        data: StickyNoteData {
            content: String::from(text),
            shape: StickyNoteShape::Rectangle,
        },
        style: None,
        position: Some(pos),
        geometry: Some(StickyNoteGeometry::WithWidthOnly {
            width: 44.0,
            // height: 500.0,
            // rotation: None,
        }),
        // parent: None,
    };

    let builder = client
        .post(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .bearer_auth(api_token)
        .json(&note);
    let _note = format!("dasda {}", serde_json::to_string_pretty(&note).unwrap());
    println!("Request sent: {_note}");

    let response = builder.send().expect(&_note);

    match response.status() {
        reqwest::StatusCode::CREATED => Some(
            response
                .json::<StickyNoteResponse>()
                .expect("Could not deserialize successfull response"),
        ),
        other => {
            let r: MiroResponseError = response
                .json()
                .expect("Could not deserialize error response");
            let err_msg = format!(
                "Error in creating sticky note: expected {}, got {}: {:?}",
                StatusCode::CREATED,
                other,
                r.message
            );
            eprintln!("{}", err_msg);
            None
        }
    }
}

pub fn create_connector(
    board: &str,
    start_id: &str,
    end_id: &str,
) -> Result<Connector, reqwest::Error> {
    let url = format!(
        "https://api.miro.com/v2-experimental/boards/{}/connectors",
        board
    );
    let api_token = env::var("MIRO_ACCESS_TOKEN").expect("env var not set");
    let client = Client::new();
    let body = json!({
         "startItem": {
              "id": start_id,
         },
         "endItem": {
              "id": end_id,
         }
    });
    let response = client
        .post(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .bearer_auth(api_token)
        .json(&body)
        .send()?;
    let connector: Connector = response.json()?;
    Ok(connector)
}
