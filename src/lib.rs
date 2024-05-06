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
    Wrapper {
        start_tag: Option<String>,
        end_tag: Option<String>,
        category: String,
        r#type: String,
        children: Vec<Node>,
        bounds: String,
    },
    Block {
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
            text: text.to_string(),
        },
    ))
}

fn basic_section_end<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>,
    key: &'a str,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    inside.pop();
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, r#type) = tag(key).context("").parse(source)?;
    let (source, _) = tuple((space0, newline)).context("").parse(source)?;
    let (source, _) = tuple((space0, newline)).context("").parse(source)?;
    let (source, children) = if *inside.last().unwrap() == "list" {
        many0(list_item_block).context("").parse(source)?
    } else {
        many0(basic_block).context("").parse(source)?
    };
    Ok((
        source,
        Node::Wrapper {
            start_tag: None,
            end_tag: Some(format!("</{}>", key)),
            category: "basic".to_string(),
            r#type: r#type.to_string(),
            children,
            bounds: "end".to_string(),
        },
    ))
}

fn basic_section_full(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    dbg!("basic_section_full");
    dbg!(source);
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = basic_section_tag.context("").parse(source)?;
    let (source, _) = tuple((space0, newline)).context("").parse(source)?;
    let (source, _) = tuple((space0, newline)).context("").parse(source)?;
    let (source, children) = many0(basic_block).context("").parse(source)?;
    Ok((
        source,
        Node::Wrapper {
            start_tag: Some(format!("<{}>", r#type)),
            end_tag: Some(format!("</{}>", r#type)),
            category: "basic".to_string(),
            r#type: r#type.to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

fn basic_section_start<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let category = "basic";
    inside.push(category);
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = basic_section_tag.context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = tuple((space0, newline)).context("").parse(source)?;
    let (source, _) = tuple((space0, newline)).context("").parse(source)?;
    let (source, mut children) = many0(alt((basic_block, |src| {
        start_or_full_section(src, inside.clone())
    })))
    .context("")
    .parse(source)?;
    let (source, end_section) = basic_section_end(source, inside, r#type)?;
    children.push(end_section);
    Ok((
        source,
        Node::Wrapper {
            start_tag: Some(format!("<{}>", r#type)),
            end_tag: None,
            category: category.to_string(),
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

fn do_parse(source: &str) -> IResult<&str, Vec<Node>, ErrorTree<&str>> {
    let inside = vec!["root"];
    let (source, results) = many1(|src| start_or_full_section(src, inside.clone()))
        .context("")
        .parse(source)?;
    Ok((source, results))
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

fn list_item_block(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    dbg!("list_item_block");
    dbg!(source);
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, _) = not(tag("//")).context("").parse(source)?;
    let (source, text) = take_until("\n\n").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Block {
            text: text.to_string(),
        },
    ))
}

fn list_item_end(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    dbg!("list_item_end");
    dbg!(source);
    let (source, _) = tag("//").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Wrapper {
            start_tag: Some("".to_string()),
            end_tag: Some("</li>".to_string()),
            category: "list_item".to_string(),
            r#type: "list_item_end".to_string(),
            children: vec![],
            bounds: "full".to_string(),
        },
    ))
}

fn list_item_full(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let (source, _) = tag("- ").context("").parse(source)?;
    let (source, children) = many0(list_item_block).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Wrapper {
            start_tag: Some("<li>".to_string()),
            end_tag: Some("</li>".to_string()),
            category: "list_item".to_string(),
            r#type: "list_item".to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

fn list_item_start(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    dbg!("list_item_start");
    dbg!(source);
    let (source, _) = tag("-/ ").context("").parse(source)?;
    let (source, mut children) = many0(alt((list_item_block, |src| {
        start_or_full_section(src, vec!["see-if-this-can-be-removed"])
    })))
    .context("")
    .parse(source)?;
    let (source, ending) = list_item_end.context("").parse(source)?;
    children.push(ending);
    Ok((
        source,
        Node::Wrapper {
            start_tag: Some("<li>".to_string()),
            end_tag: Some("".to_string()),
            category: "list_item".to_string(),
            r#type: "list_item".to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

// fn list_section_end<'a>(
//     source: &'a str,
//     mut inside: Vec<&'a str>,
//     key: &'a str,
// ) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
//     inside.pop();
//     let (source, _) = tag("-- ").context("").parse(source)?;
//     let (source, _) = tag("/").context("").parse(source)?;
//     let (source, r#type) = tag(key).context("").parse(source)?;
//     let (source, _) = tuple((space0, newline)).context("").parse(source)?;
//     let (source, _) = tuple((space0, newline)).context("").parse(source)?;
//     let (source, children) = if *inside.last().unwrap() == "list" {
//         many0(list_item_block).context("").parse(source)?
//     } else {
//         many0(basic_block).context("").parse(source)?
//     };
//     Ok((
//         source,
//         Node::Wrapper {
//             start_tag: None,
//             end_tag: Some("</ul>".to_string()),
//             category: "list".to_string(),
//             r#type: r#type.to_string(),
//             children,
//             bounds: "end".to_string(),
//         },
//     ))
// }

fn list_section_full(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = list_section_tag.context("").parse(source)?;
    let (source, _) = tuple((space0, newline)).context("").parse(source)?;
    let (source, _) = tuple((space0, newline)).context("").parse(source)?;
    let (source, children) = many0(alt((list_item_full, list_item_start)))
        .context("")
        .parse(source)?;
    Ok((
        source,
        Node::Wrapper {
            start_tag: Some("<ul>".to_string()),
            end_tag: Some("</ul>".to_string()),
            category: "list".to_string(),
            r#type: r#type.to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

// fn list_section_start<'a>(
//     source: &'a str,
//     mut inside: Vec<&'a str>,
// ) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
//     let category = "list";
//     inside.push(category);
//     let (source, _) = tag("-- ").context("").parse(source)?;
//     let (source, r#type) = list_section_tag.context("").parse(source)?;
//     let (source, _) = tag("/").context("").parse(source)?;
//     let (source, _) = tuple((space0, newline)).context("").parse(source)?;
//     let (source, _) = tuple((space0, newline)).context("").parse(source)?;
//     let (source, mut children) = many0(alt((list_item_full, |src| {
//         start_or_full_section(src, inside.clone())
//     })))
//     .context("")
//     .parse(source)?;
//     let (source, end_section) = list_section_end(source, inside.clone(), r#type)?;
//     children.push(end_section);
//     Ok((
//         source,
//         Node::Wrapper {
//             start_tag: Some("<ul>".to_string()),
//             end_tag: None,
//             category: category.to_string(),
//             r#type: r#type.to_string(),
//             children,
//             bounds: "start".to_string(),
//         },
//     ))
// }

fn list_section_tag<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, r#type) = alt((tag("list"),)).context("").parse(source)?;
    Ok((source, r#type))
}

pub fn output(ast: &Vec<Node>) -> String {
    let mut response = String::from("");
    ast.iter().for_each(|a| match a {
        Node::Wrapper {
            bounds,
            children,
            end_tag,
            start_tag,
            ..
        } => {
            if bounds == "end" {
                if let Some(s) = start_tag {
                    response.push_str(s)
                }
                if let Some(e) = end_tag {
                    response.push_str(e)
                }
                response.push_str(&output(&children));
            } else {
                if let Some(s) = start_tag {
                    response.push_str(s)
                }
                response.push_str(&output(&children));
                if let Some(e) = end_tag {
                    response.push_str(e)
                }
            }
        }
        Node::Block { text } => response.push_str(format!("<p>{}</p>", text).as_str()),
    });
    response
}

pub fn parse(source: &str) -> Result<Vec<Node>, ParserError> {
    match final_parser(do_parse)(source) {
        Ok(ast) => Ok(ast),
        Err(e) => Err(get_error(source, &e)),
    }
}

fn start_or_full_section<'a>(
    source: &'a str,
    inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    dbg!(source);
    let (source, results) = alt((
        |src| basic_section_full(src),
        |src| basic_section_start(src, inside.clone()),
        |src| list_section_full(src),
        // |src| list_section_start(src, inside.clone()),
    ))
    .context("")
    .parse(source)?;
    Ok((source, results))
}
