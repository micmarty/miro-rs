use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use serde_json::{json, Value};
use std::env;
pub mod structs;
use structs::{Connector, Position, StickyNoteResponse};

pub fn create_sticky_note(
    board: &str,
    text: &String,
    pos: Position,
    shape: &str,
) -> Result<StickyNoteResponse, reqwest::Error> {
    let url = format!("https://api.miro.com/v2/boards/{}/sticky_notes", board);
    let api_token = env::var("MIRO_ACCESS_TOKEN").expect("env var not set");
    let client = Client::new();
    let body = json!({
         "data": {
              "content": text,
              "shape": shape
         },
         "position": {
              "origin": "center",
              "x": pos.x,
              "y": pos.y
         }
    });
    let response = client
        .post(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .bearer_auth(api_token)
        .json(&body)
        .send()?;
    let note: StickyNoteResponse = response.json()?;
    Ok(note)
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
