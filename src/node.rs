#[derive(Debug)]
pub enum Node {
    Basic {
        kind: String,
        r#type: String,
        children: Vec<Node>,
        bounds: String,
    },
    Block {
        spans: String,
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
        kind: String,
        r#type: String,
        children: Vec<Node>,
        bounds: String,
    },
    Json {
        bounds: String,
        kind: String,
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
        kind: String,
        r#type: String,
        text: Option<String>,
        children: Vec<Node>,
    },
    Yaml {
        bounds: String,
        kind: String,
        r#type: String,
        data: Option<String>,
        children: Vec<Node>,
    },
}
