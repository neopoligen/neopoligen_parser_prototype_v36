use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::combinator::not;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::error::GenericErrorTree;
use nom_supreme::final_parser::final_parser;
use nom_supreme::final_parser::Location;
use nom_supreme::final_parser::RecreateContext;
use nom_supreme::parser_ext::ParserExt;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::bytes::complete::is_not;

#[derive(Debug)]
pub struct ParserError {
    pub line: usize,
    pub column: usize,
    pub remainder: String,
    #[allow(dead_code)]
    pub source: String,
    pub message: String,
}

#[derive(Debug)]
pub enum Node {
    Basic {
        start_tag: Option<String>,
        end_tag: Option<String>,
        kind: String,
        r#type: String,
        children: Vec<Node>,
        bounds: String,
    },
    Block {
        spans: String,
    },
    Json {
        bounds: String,
        kind: String,
        r#type: String,
        data: String,
    },
    Raw {
        bounds: String,
        kind: String,
        r#type: String,
        text: String,
    },
}

fn basic_block(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let (source, _) = not(tag("--")).context("").parse(source)?;
    let (source, _) = not(tag("//")).context("").parse(source)?;
    // using take_until isn't robust but works for this prototype
    let (source, text) = take_until("\n\n").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Block {
            spans: text.to_string(),
        },
    ))
}

fn basic_section_end<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>,
    key: &'a str,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    inside.pop();
    let kind = "basic";
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, r#type) = tag(key).context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(basic_block).context("").parse(source)?;
    Ok((
        source,
        Node::Basic {
            start_tag: None,
            end_tag: Some(format!("</{}>", key)),
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            children,
            bounds: "end".to_string(),
        },
    ))
}

fn basic_section_full(source: &str,
    ) -> IResult<&str, Node, ErrorTree<&str>> {
    let kind = "basic";
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = basic_section_tag.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(basic_block).context("").parse(source)?;
    Ok((
        source,
        Node::Basic {
            start_tag: Some(format!("<{}>", r#type)),
            end_tag: Some(format!("</{}>", r#type)),
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

fn basic_section_start<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "basic";
    inside.push(kind);
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = basic_section_tag.context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, mut children) = many0(alt((basic_block, |src| {
        start_or_full_section(src, inside.clone())
    })))
    .context("")
    .parse(source)?;
    let (source, end_section) = basic_section_end(source, inside.clone(), r#type)?;
    children.push(end_section);
    Ok((
        source,
        Node::Basic {
            start_tag: Some(format!("<{}>", r#type)),
            end_tag: None,
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            children,
            bounds: "start".to_string(),
        },
    ))
}

fn basic_section_tag<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, r#type) = alt((tag("div"), tag("h2"), tag("p"), tag("title")))
        .context("")
        .parse(source)?;
    Ok((source, r#type))
}

fn checklist_item_block(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let (source, _) = not(tag("--")).context("").parse(source)?;
    let (source, _) = not(tag("[")).context("").parse(source)?;
    let (source, _) = not(tag("//")).context("").parse(source)?;
    // using take_until isn't robust but works for this prototype
    let (source, text) = take_until("\n\n").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Block {
            spans: text.to_string(),
        },
    ))
}

fn checklist_item_end(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let kind = "checklist_item";
    let (source, _) = tag("//").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Basic {
            start_tag: Some("".to_string()),
            end_tag: Some("</li>".to_string()),
            kind: kind.to_string(),
            r#type: "checklist_item".to_string(),
            children: vec![],
            bounds: "end".to_string(),
        },
    ))
}

fn checklist_item_full(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let kind = "checklist";
    // NOTE: this prototype only looks for unchecked items
    let (source, _) = tag("[] ").context("").parse(source)?;
    let (source, children) = many0(checklist_item_block).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Basic {
            start_tag: Some("<li>".to_string()),
            end_tag: Some("</li>".to_string()),
            kind: kind.to_string(),
            r#type: "checklist_item".to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

fn checklist_item_start<'a>(source: &'a str,
    mut inside: Vec<&'a str>) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "checklist_item";
    inside.push(kind);
    let (source, _) = tag("[]/ ").context("").parse(source)?;
    let (source, mut children) = many0(alt((checklist_item_block, |src| {
        start_or_full_section(src, inside.clone())
    })))
    .context("")
    .parse(source)?;
    let (source, ending) = checklist_item_end.context("").parse(source)?;
    children.push(ending);
    Ok((
        source,
        Node::Basic {
            start_tag: Some("<li>".to_string()),
            end_tag: Some("".to_string()),
            kind: kind.to_string(),
            r#type: "checklist_item".to_string(),
            children,
            bounds: "start".to_string(),
        },
    ))
}

fn checklist_section_full<'a>(source: &'a str,
    mut inside: Vec<&'a str>) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "checklist";
    inside.push(kind);
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = checklist_section_tag.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(alt((
        |src| checklist_item_full(src), 
        |src| checklist_item_start(src, inside.clone())
    )))
        .context("")
        .parse(source)?;
    Ok((
        source,
        Node::Basic {
            start_tag: Some("<ul>".to_string()),
            end_tag: Some("</ul>".to_string()),
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

fn checklist_section_tag<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, r#type) = alt((tag("checklist"),)).context("").parse(source)?;
    Ok((source, r#type))
}

fn empty_until_newline_or_eof<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, _) = alt((
        tuple((space0, newline.map(|_| ""))),
        tuple((multispace0, eof.map(|_| "")))
    ))
    .context("")
    .parse(source)?;
    Ok((source, ""))
}

fn generic_section_end<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>,
    key: &'a str,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "generic";
    inside.pop();
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, r#type) = tag(key).context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(basic_block).context("").parse(source)?;
    Ok((
        source,
        Node::Basic {
            start_tag: None,
            end_tag: Some(format!("</{}>", key)),
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            children,
            bounds: "end".to_string(),
        },
    ))
}

fn generic_section_full(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let kind = "generic";
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = is_not(" /\n").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(basic_block).context("").parse(source)?;
    Ok((
        source,
        Node::Basic {
            start_tag: Some(format!("<{}>", r#type)),
            end_tag: Some(format!("</{}>", r#type)),
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

fn generic_section_start<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "generic";
    inside.push(kind);
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = is_not(" /\n").context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, mut children) = many0(alt((basic_block, |src| {
        start_or_full_section(src, inside.clone())
    })))
    .context("")
    .parse(source)?;
    let (source, end_section) = generic_section_end(source, inside.clone(), r#type)?;
    children.push(end_section);
    Ok((
        source,
        Node::Basic {
            start_tag: Some(format!("<{}>", r#type)),
            end_tag: None,
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            children,
            bounds: "start".to_string(),
        },
    ))
}

fn get_error(content: &str, tree: &ErrorTree<&str>) -> ParserError {
    match tree {
        GenericErrorTree::Base { location, kind } => {
            let details = Location::recreate_context(content, location);
            ParserError {
                line: details.line,
                column: details.column,
                source: content.to_string(),
                remainder: location.to_string(),
                message: kind.to_string(),
            }
        }
        GenericErrorTree::Stack { contexts, .. } => {
            let context = contexts[0];
            let details = Location::recreate_context(content, context.0);
            ParserError {
                line: details.line,
                column: details.column,
                source: content.to_string(),
                remainder: context.0.to_string(),
                message: context.1.to_string(),
            }
        }
        GenericErrorTree::Alt(items) => get_error(content, &items[0]),
    }
}


fn json_section_end() {
    // TODO
}

fn json_section_full(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let kind = "json";
    // Note: this is not actually converted to json for the prototype, but
    // it will be in the full code
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = json_section_tag.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = many0(empty_until_newline_or_eof).context("").parse(source)?;
    let (source, data) = alt((take_until("\n--"), rest)).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Json {
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            data: data.trim_end().to_string(),
            bounds: "full".to_string(),
        },
    ))
}

fn json_section_start<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "json";
    inside.push(kind);
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = json_section_tag.context("").parse(source)?;
    let end_key = format!("-- /{}", r#type);
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = many0(empty_until_newline_or_eof).context("").parse(source)?;
    let (source, data) =  take_until(end_key.as_str()).context("").parse(source)?;
    let (source, _) = tag(end_key.as_str()).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Json {
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            data: data.trim_end().to_string(),
            bounds: "full".to_string(),
        },
    ))
}

fn json_section_tag<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, r#type) = alt((tag("metadata"), tag("metadata")))
        .context("")
        .parse(source)?;
    Ok((source, r#type))
}

fn list_item_block(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, _) = not(tag("//")).context("").parse(source)?;
    // using take_until isn't robust but works for this prototype
    let (source, text) = take_until("\n\n").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Block {
            spans: text.to_string(),
        },
    ))
}

fn list_item_end<'a>(source: &'a str,
    mut inside: Vec<&'a str>) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "list_item";
    inside.pop();
    let (source, _) = tag("//").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Basic {
            start_tag: Some("".to_string()),
            end_tag: Some("</li>".to_string()),
            kind: kind.to_string(),
            r#type: "list_item".to_string(),
            children: vec![],
            bounds: "end".to_string(),
        },
    ))
}

fn list_item_full(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let kind = "list_item";
    let (source, _) = tag("- ").context("").parse(source)?;
    let (source, children) = many0(list_item_block).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Basic {
            start_tag: Some("<li>".to_string()),
            end_tag: Some("</li>".to_string()),
            kind: kind.to_string(),
            r#type: "list_item".to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

fn list_item_start<'a>(source: &'a str,
    mut inside: Vec<&'a str>) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "list_item";
    inside.push(kind);
    let (source, _) = tag("-/ ").context("").parse(source)?;
    let (source, mut children) = many0(alt((list_item_block, |src| {
        start_or_full_section(src, inside.clone())
    })))
    .context("")
    .parse(source)?;
    let (source, ending) = (|src| list_item_end(src, inside.clone())).context("").parse(source)?;
    children.push(ending);
    Ok((
        source,
        Node::Basic {
            start_tag: Some("<li>".to_string()),
            end_tag: Some("".to_string()),
            kind: kind.to_string(),
            r#type: "list_item".to_string(),
            children,
            bounds: "start".to_string(),
        },
    ))
}

fn list_section_full<'a>(source: &'a str,
    mut inside: Vec<&'a str>) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "list";
    inside.push(kind);
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = list_section_tag.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(alt((list_item_full, |src| list_item_start(src, inside.clone()))))
        .context("")
        .parse(source)?;
    Ok((
        source,
        Node::Basic {
            start_tag: Some("<ul>".to_string()),
            end_tag: Some("</ul>".to_string()),
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

fn list_section_tag<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, r#type) = alt((tag("list"),)).context("").parse(source)?;
    Ok((source, r#type))
}

pub fn output(ast: &Vec<Node>) -> String {
    let mut response = String::from("");
    ast.iter().for_each(|a| match a {
        Node::Basic {
            bounds,
            children,
            kind,
            r#type,
            ..
        } => {
            if kind == "basic" {
                if bounds == "full" {
                    response.push_str("<div class=\"");
                    response.push_str(kind);
                    response.push_str("-");
                    response.push_str(bounds);
                    response.push_str("-");
                    response.push_str(r#type);
                    response.push_str("\">");
                    response.push_str(&output(&children));
                    response.push_str("</div>");
                }
                if bounds == "start" {
                    response.push_str("<div class=\"");
                    response.push_str(kind);
                    response.push_str("-");
                    response.push_str(bounds);
                    response.push_str("-");
                    response.push_str(r#type);
                    response.push_str("\">");
                    response.push_str(&output(&children));
                }
                if bounds == "end" {
                    response.push_str("</div>");
                    response.push_str("<div class=\"");
                    response.push_str(kind);
                    response.push_str("-");
                    response.push_str(bounds);
                    response.push_str("-");
                    response.push_str(r#type);
                    response.push_str("\">");
                    response.push_str(&output(&children));
                    response.push_str("</div>");
                }
            } else if kind == "list" {
                if bounds == "full" {
                    response.push_str("<ul class=\"");
                    response.push_str(kind);
                    response.push_str("-");
                    response.push_str(bounds);
                    response.push_str("-");
                    response.push_str(r#type);
                    response.push_str("\">");
                    response.push_str(&output(&children));
                    response.push_str("</ul>");
                }
                if bounds == "start" {
                    response.push_str("<ul class=\"");
                    response.push_str(kind);
                    response.push_str("-");
                    response.push_str(bounds);
                    response.push_str("-");
                    response.push_str(r#type);
                    response.push_str("\">");
                    response.push_str(&output(&children));
                }
                if bounds == "end" {
                    response.push_str("</ul>");
                    response.push_str("<div class=\"");
                    response.push_str(kind);
                    response.push_str("-");
                    response.push_str(bounds);
                    response.push_str("-");
                    response.push_str(r#type);
                    response.push_str("\">");
                    response.push_str(&output(&children));
                    response.push_str("</div>");
                }
            } else if kind == "list_item" {
                if bounds == "full" {
                    response.push_str("<li class=\"");
                    response.push_str(kind);
                    response.push_str("-");
                    response.push_str(bounds);
                    response.push_str("-");
                    response.push_str(r#type);
                    response.push_str("\">");
                    response.push_str(&output(&children));
                    response.push_str("</li>");
                }
                if bounds == "start" {
                    response.push_str("<li class=\"");
                    response.push_str(kind);
                    response.push_str("-");
                    response.push_str(bounds);
                    response.push_str("-");
                    response.push_str(r#type);
                    response.push_str("\">");
                    response.push_str(&output(&children));
                }
                if bounds == "end" {
                    response.push_str("</li>");
                    response.push_str("<li class=\"");
                    response.push_str(kind);
                    response.push_str("-");
                    response.push_str(bounds);
                    response.push_str("-");
                    response.push_str(r#type);
                    response.push_str("\">");
                    response.push_str(&output(&children));
                    response.push_str("</div>");
                }
            }
        }
        Node::Block { spans } => response.push_str(format!("<p>{}</p>", spans).as_str()),
        Node::Json { data, r#type, .. } => response.push_str(format!("<h2>{}</h2><pre>{}</pre>", r#type, data).as_str()),
        Node::Raw { text, r#type, .. } => response.push_str(format!("<h2>{}</h2><pre>{}</pre>", r#type, text).as_str()),
    });
    response
}

pub fn parse(source: &str) -> Result<Vec<Node>, ParserError> {
    match final_parser(parse_runner)(source) {
        Ok(ast) => Ok(ast),
        Err(e) => Err(get_error(source, &e)),
    }
}

fn parse_runner(source: &str) -> IResult<&str, Vec<Node>, ErrorTree<&str>> {
    let inside = vec!["root"];
    let (source, results) = many1(|src| start_or_full_section(src, inside.clone()))
        .context("")
        .parse(source)?;
    Ok((source, results))
}

fn raw_secction_end() {
    // TODO
}

fn raw_section_full(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let kind = "raw";
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = raw_section_tag.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = many0(empty_until_newline_or_eof).context("").parse(source)?;
    let (source, text) = alt((take_until("\n--"), rest)).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Raw {
            bounds: "full".to_string(),
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            text: text.trim_end().to_string(), 
        },
    ))
}

fn raw_section_start<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "raw";
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = raw_section_tag.context("").parse(source)?;
    let end_key = format!("-- /{}", r#type);
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = many0(empty_until_newline_or_eof).context("").parse(source)?;
    let (source, text) =  take_until(end_key.as_str()).context("").parse(source)?;
    let (source, _) = tag(end_key.as_str()).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Raw {
            bounds: "full".to_string(),
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            text: text.trim_end().to_string(), 
        },
    ))
}

fn raw_section_tag<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, r#type) = alt((tag("pre"), tag("code")))
        .context("")
        .parse(source)?;
    Ok((source, r#type))
}

fn start_or_full_section<'a>(
    source: &'a str,
    inside: Vec<&'a str>
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, results) = alt((
        |src| basic_section_full(src),
        |src| basic_section_start(src, inside.clone()),
        |src| checklist_section_full(src, inside.clone()),
        |src| json_section_full(src),
        |src| json_section_start(src, inside.clone()),
        |src| list_section_full(src, inside.clone()),
        |src| raw_section_full(src),
        |src| raw_section_start(src, inside.clone()),
        // make sure generic is last
        |src| generic_section_full(src),
        |src| generic_section_start(src, inside.clone()),
    ))
    .context("")
    .parse(source)?;
    Ok((source, results))
}

