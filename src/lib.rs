pub mod block;
pub mod section;
pub mod span;

use crate::node::Node;
use crate::section::*;
use crate::span::Span;
use nom::multi::many1;
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
pub struct Sections {
    pub basic: Vec<String>,
    pub checklist: Vec<String>,
    pub comment: Vec<String>,
    pub detail: Vec<String>,
    pub generic: Vec<String>,
    pub json: Vec<String>,
    pub list: Vec<String>,
    pub raw: Vec<String>,
    pub table: Vec<String>,
    pub yaml: Vec<String>,
}

#[derive(Debug)]
pub struct ParserError {
    pub line: usize,
    pub column: usize,
    pub remainder: String,
    #[allow(dead_code)]
    pub source: String,
    pub message: String,
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

pub fn output(ast: &Vec<Node>) -> String {
    let mut response = String::from("");
    ast.iter().for_each(|a| match a {
        Node::Basic {
            bounds,
            children,
            r#type,
            ..
        } => {
            if bounds == "full" {
                response.push_str("<div class=\"");
                response.push_str("basic-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(&output(&children));
                response.push_str("</div>");
            }
            if bounds == "start" {
                response.push_str("<div class=\"");
                response.push_str("basic-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(&output(&children));
            }
            if bounds == "end" {
                response.push_str("<!-- ");
                response.push_str("basic-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str(" -->");
                response.push_str("</div>");
                response.push_str(&output(&children));
            }
        }

        //Node::Block { spans } => response.push_str(format!("<p>{}</p>", spans).as_str()),
        Node::Block { spans } => {
            response.push_str("<p>");
            response.push_str(output_spans(spans).as_str());
            response.push_str("</p>");
        }
        Node::Checklist {
            bounds,
            children,
            r#type,
            ..
        } => {
            if bounds == "full" {
                response.push_str("<ul class=\"checklist-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(&output(&children));
                response.push_str("</ul>");
            } else if bounds == "start" {
                response.push_str("<ul class=\"checklist-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(&output(&children));
            } else if bounds == "end" {
                response.push_str("<!-- checklist-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str(" -->");
                response.push_str("</ul>");
                response.push_str(&output(&children));
            }
        }

        Node::ChecklistItem {
            children, status, ..
        } => {
            response.push_str(
                format!("<li class=\"status-{}\">", status.to_string().as_str()).as_str(),
            );
            response.push_str(&output(&children));
            response.push_str("</li>");
        }

        Node::Comment {
            bounds,
            r#type,
            children,
            ..
        } => {
            if bounds == "end" {
                response.push_str(format!("<!-- comment-{}-{} -->", bounds, r#type).as_str());
                response.push_str(&output(&children));
            } else if bounds == "full" {
                response.push_str(format!("<!-- comment-{}-{} -->", bounds, r#type).as_str());
            } else if bounds == "start" {
                response.push_str(format!("<!-- comment-{}-{} -->", bounds, r#type).as_str());
                response.push_str(&output(&children));
            }
        }

        Node::Generic {
            bounds,
            children,
            r#type,
            ..
        } => {
            if bounds == "full" {
                response
                    .push_str(format!("<div class=\"generic-{}-{}\">", bounds, r#type).as_str());
                response.push_str(&output(&children));
                response.push_str("</div>");
            }
            if bounds == "start" {
                response
                    .push_str(format!("<div class=\"generic-{}-{}\">", bounds, r#type).as_str());
                response.push_str(&output(&children));
            }
            if bounds == "end" {
                response.push_str(format!("<!-- generic-{}-{} -->", bounds, r#type).as_str());
                response.push_str("</div>");
                response.push_str(&output(&children));
            }
        }

        Node::Json {
            bounds,
            children,
            data,
            r#type,
            ..
        } => {
            if bounds == "end" {
                response.push_str(format!("<!-- json-end-{} -->", r#type).as_str());
                response.push_str(&output(&children));
            } else if bounds == "full" {
                response.push_str(
                    format!("<!-- json-full-{} -->{}", r#type, data.clone().unwrap()).as_str(),
                );
            } else if bounds == "start" {
                response.push_str(
                    format!("<!-- json-start-{} -->{}", r#type, data.clone().unwrap()).as_str(),
                );
                response.push_str(&output(&children));
            }
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
                response.push_str("<!-- list ");
                response.push_str("-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str(" -->");
                response.push_str("</ul>");
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
            bounds,
            children,
        } => {
            if bounds == "full" {
                response.push_str("<pre class=\"");
                response.push_str("raw-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(text.clone().unwrap().as_str());
                response.push_str("</pre>");
            } else if bounds == "start" {
                response.push_str("<pre class=\"");
                response.push_str("raw-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str("\">");
                response.push_str(text.clone().unwrap().as_str());
                response.push_str(&output(&children));
            } else if bounds == "end" {
                response.push_str("</pre>");
                response.push_str("<!-- ");
                response.push_str("raw-");
                response.push_str(bounds);
                response.push_str("-");
                response.push_str(r#type);
                response.push_str(" -->");
                response.push_str(&output(&children));
            }
        }
        Node::TagFinderInit => {}
        Node::Yaml {
            bounds,
            children,
            data,
            r#type,
            ..
        } => {
            if bounds == "end" {
                response.push_str(format!("<!-- yaml-end-{} -->", r#type).as_str());
                response.push_str(&output(&children));
            } else if bounds == "full" {
                response.push_str(
                    format!("<!-- yaml-full-{} -->{}", r#type, data.clone().unwrap()).as_str(),
                );
            } else if bounds == "start" {
                response.push_str(
                    format!("<!-- yaml-start-{} -->{}", r#type, data.clone().unwrap()).as_str(),
                );
                response.push_str(&output(&children));
            }
        }
    });
    response
}

pub fn output_spans(spans: &Vec<Span>) -> String {
    let mut response = String::from("");
    spans.iter().for_each(|span| match span {
        Span::Button { attrs, flags, text } => {
            response.push_str(format!("<button").as_str());
            attrs.iter().for_each(|attr| {
                response.push_str(format!(" {}=\"{}\"", attr.0.as_str(), attr.1.as_str()).as_str());
            });
            flags.iter().for_each(|flag| {
                response.push_str(format!(" {}", flag).as_str());
            });
            response.push_str(format!(">{}</button>", text).as_str());
        }
        Span::Code { attrs, flags, text } => {
            response.push_str(format!("<code").as_str());
            attrs.iter().for_each(|attr| {
                response.push_str(format!(" {}=\"{}\"", attr.0.as_str(), attr.1.as_str()).as_str());
            });
            flags.iter().for_each(|flag| {
                response.push_str(format!(" {}", flag).as_str());
            });
            response.push_str(format!(">{}</code>", text).as_str());
        }
        Span::Em { attrs, flags, text } => {
            response.push_str(format!("<em").as_str());
            attrs.iter().for_each(|attr| {
                response.push_str(format!(" {}=\"{}\"", attr.0.as_str(), attr.1.as_str()).as_str());
            });
            flags.iter().for_each(|flag| {
                response.push_str(format!(" {}", flag).as_str());
            });
            response.push_str(format!(">{}</em>", text).as_str());
        }
        Span::KnownSpan {
            r#type,
            spans,
            attrs,
            flags,
        } => {
            response.push_str(format!("<{}", r#type).as_str());
            attrs.iter().for_each(|attr| {
                response.push_str(format!(" {}=\"{}\"", attr.0.as_str(), attr.1.as_str()).as_str());
            });
            flags.iter().for_each(|flag| {
                response.push_str(format!(" {}", flag).as_str());
            });
            response.push_str(format!(">{}</{}>", output_spans(spans), r#type).as_str());
        }
        Span::Newline { .. } => {
            response.push_str(" ");
        }
        Span::Space { .. } => {
            response.push_str(" ");
        }
        Span::Strong { attrs, flags, text } => {
            response.push_str(format!("<strong").as_str());
            attrs.iter().for_each(|attr| {
                response.push_str(format!(" {}=\"{}\"", attr.0.as_str(), attr.1.as_str()).as_str());
            });
            flags.iter().for_each(|flag| {
                response.push_str(format!(" {}", flag).as_str());
            });
            response.push_str(format!(">{}</strong>", text).as_str());
        }
        Span::UnknownSpan {
            r#type,
            spans,
            attrs,
            flags,
        } => {
            response.push_str(format!("<span").as_str());
            attrs.iter().for_each(|attr| {
                response.push_str(format!(" {}=\"{}\"", attr.0.as_str(), attr.1.as_str()).as_str());
            });
            flags.iter().for_each(|flag| {
                response.push_str(format!(" {}", flag).as_str());
            });
            response.push_str(format!(">{}</span>", output_spans(spans)).as_str());
        }
        Span::WordPart { text } => {
            response.push_str(text);
        }
    });
    response
}

pub fn parse(
    source: &str,
    sections: &Sections,
    spans: &Vec<String>,
) -> Result<Vec<Node>, ParserError> {
    match final_parser(|src| parse_runner(src, sections, spans))(source) {
        Ok(ast) => Ok(ast),
        Err(e) => Err(get_error(source, &e)),
    }
}

fn parse_runner<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Vec<Node>, ErrorTree<&'a str>> {
    let (source, results) = many1(|src| start_or_full_section(src, &sections, &spans))
        .context("")
        .parse(source)?;
    Ok((source, results))
}
