use serde::{Deserialize, Serialize};
use serde_json::Value;

// use chrono::{DateTime, Utc};
// use derivative::Derivative;
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

// square rectangle
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum StickyNoteShape {
    Square,
    Rectangle,
}

impl Default for StickyNoteShape {
    fn default() -> Self {
        Self::Square
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyNoteData {
    pub shape: StickyNoteShape,
    pub content: String,
}

impl StickyNoteData {
    fn new(text: String) -> Self {
        Self {
            content: text,
            shape: Default::default(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StickyNoteResponse {
    pub id: String,
    pub data: Value,
    pub style: StickyNoteStyle,
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
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyNoteCreate {
    pub data: StickyNoteData,
    pub style: Option<StickyNoteStyle>,
    // #[serde(default)]
    pub position: Option<Position>,
    // #[serde(default)]
    pub geometry: Option<StickyNoteGeometry>,
    // pub parent: Option<Value>,
}

pub trait Positional {
    fn at(&mut self, pos: Position);
}

impl Positional for StickyNoteCreate {
    fn at(&mut self, pos: Position) {
        self.position = Some(pos);
    }
}

impl StickyNoteCreate {
    // TODO refactor to builder pattern
    pub fn with_text(text: String) -> Self {
        Self {
            data: StickyNoteData::new(text),
            style: Default::default(),
            position: Default::default(),
            geometry: Default::default(),
        }
    }
}

// impl Default for StickyNoteCreate {
//     fn default() -> Self {
//         Self {
//             data: StickyNoteData::new(String::from("default text")),
//             ..Default::default()
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Captions {
    content: String,
    position: String,
    text_align_vertical: VerticalAlignment,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    #[serde(rename = "type")]
    object_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub links: Value, //TODO
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StickyNoteStyle {
    pub fill_color: Color,
    pub text_align: HorizontalAlignment,
    pub text_align_vertical: Option<VerticalAlignment>,
}

impl Default for StickyNoteStyle {
    fn default() -> Self {
        Self {
            fill_color: Color::Yellow,
            // TODO replace with ..Default::default()
            text_align: HorizontalAlignment::Center,
            text_align_vertical: Some(VerticalAlignment::Middle),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    // TODO origin can only be "center", quote: "Currently, only one option is supported".
}

impl Default for Position {
    fn default() -> Position {
        Position { x: 0.0, y: 0.0 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Geometry {
    pub width: f32,
    // pub height: f32,
    // pub rotation: Option<f32>,
}
impl Default for Geometry {
    fn default() -> Geometry {
        Geometry {
            width: 200.0,
            // height: 200.0,
            // rotation: Some(0.0),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct MiroResponseError {
    pub code: String,
    pub message: String,
    pub status: i32,
    #[serde(rename = "type")]
    pub error_type: String,
}
// You can set either the width or height. You cannot set both the at the same time
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum StickyNoteGeometry {
    WithHeight { height: f32 },
    WithWidth { width: f32 },
}

impl Default for StickyNoteGeometry {
    fn default() -> Self {
        StickyNoteGeometry::WithWidth { width: 200.0 }
    }
}
