use crate::span::Span;

#[derive(Debug)]
pub enum Node {
    Basic {
        r#type: String,
        children: Vec<Node>,
        bounds: String,
    },
    Block {
        spans: Vec<Span>,
    },
    Checklist {
        r#type: String,
        children: Vec<Node>,
        bounds: String,
    },
    ChecklistItem {
        children: Vec<Node>,
        status: bool,
        status_value: Option<String>,
    },
    Comment {
        bounds: String,
        r#type: String,
        text: Option<String>,
        children: Vec<Node>,
    },
    Generic {
        r#type: String,
        children: Vec<Node>,
        bounds: String,
    },
    Json {
        bounds: String,
        r#type: String,
        data: Option<String>,
        children: Vec<Node>,
    },
    List {
        r#type: String,
        children: Vec<Node>,
        bounds: String,
    },
    ListItem {
        children: Vec<Node>,
    },
    Raw {
        bounds: String,
        r#type: String,
        text: Option<String>,
        children: Vec<Node>,
    },
    Yaml {
        bounds: String,
        r#type: String,
        data: Option<String>,
        children: Vec<Node>,
    },
}
