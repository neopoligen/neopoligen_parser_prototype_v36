pub mod basic;
pub mod checklist;
pub mod generic;
pub mod list;
pub mod node;
pub mod raw;
pub mod section;

use crate::node::Node;
use crate::section::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::rest;
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

// NOTE: Not sure if the 'inside' stuff is needed. I started
// adding it, then realized I didn't need it for lists
// in the way I thought. I'm leaving it here for now. Can
// remove after everything else is done if it's not needed

#[derive(Debug)]
pub struct ParserError {
    pub line: usize,
    pub column: usize,
    pub remainder: String,
    #[allow(dead_code)]
    pub source: String,
    pub message: String,
}
// fn checklist_item_block(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
//     let (source, _) = not(tag("--")).context("").parse(source)?;
//     let (source, _) = not(tag("[")).context("").parse(source)?;
//     let (source, _) = not(tag("//")).context("").parse(source)?;
//     // using take_until isn't robust but works for this prototype
//     let (source, text) = take_until("\n\n").context("").parse(source)?;
//     let (source, _) = multispace0.context("").parse(source)?;
//     Ok((
//         source,
//         Node::Block {
//             spans: text.to_string(),
//         },
//     ))
// }

// fn checklist_item_end(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
//     let kind = "checklist_item";
//     let (source, _) = tag("//").context("").parse(source)?;
//     let (source, _) = multispace0.context("").parse(source)?;
//     Ok((
//         source,
//         Node::ChecklistItem {
//             r#type: "checklist_item".to_string(),
//             children: vec![],
//             bounds: "end".to_string(),
//         },
//     ))
// }

// fn checklist_item_full(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
//     let kind = "checklist";
//     // NOTE: this prototype only looks for unchecked items
//     let (source, _) = tag("[] ").context("").parse(source)?;
//     let (source, children) = many0(checklist_item_block).context("").parse(source)?;
//     let (source, _) = multispace0.context("").parse(source)?;
//     Ok((
//         source,
//         Node::Checklist {
//             r#type: "checklist_item".to_string(),
//             children,
//             bounds: "full".to_string(),
//         },
//     ))
// }

// fn checklist_item_start<'a>(
//     source: &'a str,
//     mut inside: Vec<&'a str>,
// ) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
//     let kind = "checklist_item";
//     inside.push(kind);
//     let (source, _) = tag("[]/ ").context("").parse(source)?;
//     let (source, mut children) = many0(alt((checklist_item_block, |src| {
//         start_or_full_section(src, inside.clone())
//     })))
//     .context("")
//     .parse(source)?;
//     let (source, ending) = checklist_item_end.context("").parse(source)?;
//     children.push(ending);
//     Ok((
//         source,
//         Node::Checklist {
//             r#type: "checklist_item".to_string(),
//             children,
//             bounds: "start".to_string(),
//         },
//     ))
// }

// fn checklist_section_full<'a>(
//     source: &'a str,
//     mut inside: Vec<&'a str>,
// ) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
//     let kind = "checklist";
//     inside.push(kind);
//     let (source, _) = tag("-- ").context("").parse(source)?;
//     let (source, r#type) = checklist_section_tag.context("").parse(source)?;
//     let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
//     let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
//     let (source, _) = multispace0.context("").parse(source)?;
//     let (source, children) = many0(alt((
//         |src| checklist_item_full(src),
//         |src| checklist_item_start(src, inside.clone()),
//     )))
//     .context("")
//     .parse(source)?;
//     Ok((
//         source,
//         Node::Checklist {
//             r#type: r#type.to_string(),
//             children,
//             bounds: "full".to_string(),
//         },
//     ))
// }

// fn checklist_section_tag<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
//     let (source, r#type) = alt((tag("checklist"),)).context("").parse(source)?;
//     Ok((source, r#type))
// }
//
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
                    response.push_str("<!-- ");
                    response.push_str(kind);
                    response.push_str("-");
                    response.push_str(bounds);
                    response.push_str("-");
                    response.push_str(r#type);
                    response.push_str(" -->");
                    response.push_str(&output(&children));
                }
            // } else if kind == "list" {
            //     if bounds == "full" {
            //         response.push_str("<ul class=\"");
            //         response.push_str(kind);
            //         response.push_str("-");
            //         response.push_str(bounds);
            //         response.push_str("-");
            //         response.push_str(r#type);
            //         response.push_str("\">");
            //         response.push_str(&output(&children));
            //         response.push_str("</ul>");
            //     }
            //     if bounds == "start" {
            //         response.push_str("<ul class=\"");
            //         response.push_str(kind);
            //         response.push_str("-");
            //         response.push_str(bounds);
            //         response.push_str("-");
            //         response.push_str(r#type);
            //         response.push_str("\">");
            //         response.push_str(&output(&children));
            //     }
            //     if bounds == "end" {
            //         response.push_str("</ul>");
            //         response.push_str("<div class=\"");
            //         response.push_str(kind);
            //         response.push_str("-");
            //         response.push_str(bounds);
            //         response.push_str("-");
            //         response.push_str(r#type);
            //         response.push_str("\">");
            //         response.push_str(&output(&children));
            //         response.push_str("</div>");
            //     }
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
                    response.push_str("<!-- ");
                    response.push_str(kind);
                    response.push_str("-");
                    response.push_str(bounds);
                    response.push_str("-");
                    response.push_str(r#type);
                    response.push_str(" -->");
                }
            }
        }
        Node::Block { spans } => response.push_str(format!("<p>{}</p>", spans).as_str()),

        Node::Checklist {
            bounds,
            children,
            r#type,
            ..
        } => {
            if bounds == "full" {
                response.push_str("<ul class=\"checklist");
                response.push_str("-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(&output(&children));
                response.push_str("</ul>");
            } else if bounds == "start" {
                response.push_str("<ul class=\"checklist");
                response.push_str("-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(&output(&children));
            } else if bounds == "end" {
                response.push_str("</ul>");
                response.push_str("<div class=\"");
                response.push_str("-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(&output(&children));
                response.push_str("</div>");
            }
        }

        Node::ChecklistItem { .. } => {
            response.push_str("TODO: ChecklistItem");
        }

        Node::Json { data, r#type, .. } => {
            response.push_str(format!("<h2>{}</h2><pre>{}</pre>", r#type, data).as_str())
        }

        Node::List {
            bounds,
            children,
            r#type,
        } => {
            if bounds == "full" {
                response.push_str("<ul class=\"list");
                response.push_str("-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(&output(&children));
                response.push_str("</ul>");
            }
            if bounds == "start" {
                response.push_str("<ul class=\"list");
                response.push_str("-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(&output(&children));
            }
            if bounds == "end" {
                response.push_str("</ul>");
                response.push_str("<!-- list ");
                response.push_str("-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str(" -->");
                response.push_str(&output(&children));
            }
        }

        Node::ListItem { children } => {
            response.push_str("<li>");
            response.push_str(&output(&children));
            response.push_str("</li>");
        }

        Node::Raw {
            text,
            r#type,
            kind,
            bounds,
            children,
        } => {
            if bounds == "full" {
                response.push_str("<pre class=\"");
                response.push_str(kind);
                response.push_str("-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(text.clone().unwrap().as_str());
                response.push_str("</pre>");
            } else if bounds == "start" {
                response.push_str("<pre class=\"");
                response.push_str(kind);
                response.push_str("-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(text.clone().unwrap().as_str());
                response.push_str(&output(&children));
            } else if bounds == "end" {
                response.push_str("</pre>");
                response.push_str("<!-- ");
                response.push_str(kind);
                response.push_str("-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str(" -->");
                response.push_str(&output(&children));
            }
        }
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
