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
    Json {
        bounds: String,
        kind: String,
        r#type: String,
        data: String,
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
}
