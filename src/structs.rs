use serde::{Deserialize, Serialize};
use serde_json::Value;

// use chrono::{DateTime, Utc};
use strum::IntoStaticStr;
// use strum::IntoStaticStr;

// Shape: straight elbowed curved
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Shape {
    Straight,
    Elbowed,
    Curved,
}
// Color: light_yellow, yellow, orange, light_green, green, dark_green, cyan, light_pink, pink, violet, red, light_blue, blue, dark_blue, black
#[derive(
    Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, strum_macros::Display, IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Color {
    LightYellow,
    Yellow,
    Orange,
    LightGreen,
    Green,
    DarkGreen,
    Cyan,
    LightPink,
    Pink,
    Violet,
    Red,
    LightBlue,
    Blue,
    DarkBlue,
    Black,
}
// Horizontal alignment: left right center
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum HorizontalAlignment {
    Left,
    Right,
    Center,
}

// Vertical alignment: top middle bottomr
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

// Stroke cap: none stealth diamond diamond_filled oval oval_filled arrow triangle triangle_filled erd_one erd_many erd_only_one erd_zero_or_one erd_one_or_many erd_zero_or_many
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum StrokeCap {
    None,
    Steath,
    Diamond,
    DiamondFilled,
    Oval,
    OvalFilled,
    Arrow,
    Triangle,
    TriangleFilled,
    ErdOne,
    ErdMany,
    ErdOnlyOne,
    ErdZeroOrOne,
    ErdOneOrMany,
    ErdZeroOrMany,
}

// normal dotted dashed
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum StrokeStyle {
    Normal,
    Dotted,
    Dashed,
}

// horizontal aligned
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TextOrientation {
    Horizontal,
    Aligned,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyNoteResponse {
    pub id: String,
    pub data: Value,
    pub style: CardStyle,
    pub position: Position,
    pub geometry: Geometry,
    pub created_at: String,
    pub created_by: User,
    pub modified_at: String,
    pub modified_by: User,
    pub parent: Option<Value>,
    pub links: Value,

    #[serde(rename = "type")]
    pub object_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Captions {
    content: String,
    position: String,
    text_align_vertical: VerticalAlignment,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    #[serde(rename = "type")]
    object_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub links: Value, //TODO
    pub position: Position,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connector {
    pub id: String,
    pub captions: Vec<String>,
    pub created_at: String,
    pub created_by: User,
    pub end_item: Item,
    pub start_item: Item,
    pub is_supported: bool,
    pub links: Value,
    pub modified_at: String,
    pub modified_by: User,
    pub shape: Shape,
    pub style: ConnectorStyle,
    #[serde(rename = "type")]
    pub object_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardStyle {
    pub fill_color: Color,
    pub text_align: HorizontalAlignment,
    pub test_align_vertical: Option<VerticalAlignment>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorStyle {
    pub color: String, // hex: #1a1a1a
    pub font_size: String,
    pub start_stroke_cap: StrokeCap,
    pub end_stroke_cap: StrokeCap,
    pub stroke_color: String, // hex: #000000
    pub stroke_style: StrokeStyle,
    pub stroke_width: String, // 1.0
    pub text_orientation: TextOrientation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
#[derive(Deserialize)]
pub struct Geometry {
    pub width: f32,
    pub height: f32,
    pub rotation: Option<f32>,
}
