use crate::block::*;
use crate::node::Node;
use crate::section::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::not;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn list_item_block(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, spans) = many0(span_finder).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((source, Node::Block { spans }))
}

pub fn list_item(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let (source, _) = tag("- ").context("").parse(source)?;
    let (source, children) = many0(list_item_block).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((source, Node::ListItem { children }))
}

pub fn list_item_with_sections<'a>(
    source: &'a str,
    inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, _) = tag("- ").context("").parse(source)?;
    let (source, children) = many0(alt((list_item_block, |src| {
        start_or_full_section(src, inside.clone())
    })))
    .context("")
    .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((source, Node::ListItem { children }))
}

pub fn list_section_end<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>,
    key: &'a str,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    inside.pop();
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, r#type) = tag(key).context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(block_of_end_content).context("").parse(source)?;
    Ok((
        source,
        Node::List {
            r#type: r#type.to_string(),
            children,
            bounds: "end".to_string(),
        },
    ))

    // if *inside.last().unwrap() == "list" {
    //     let (source, children) = many0(list_item).context("").parse(source)?;
    //     Ok((
    //         source,
    //         Node::List {
    //             r#type: r#type.to_string(),
    //             children,
    //             bounds: "end".to_string(),
    //         },
    //     ))
    // } else {
    //     let (source, children) = many0(block_of_anything).context("").parse(source)?;
    //     Ok((
    //         source,
    //         Node::List {
    //             r#type: r#type.to_string(),
    //             children,
    //             bounds: "end".to_string(),
    //         },
    //     ))
    // }
}

pub fn list_section_full<'a>(
    source: &'a str,
    mut _inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = list_section_tag.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(list_item).context("").parse(source)?;
    Ok((
        source,
        Node::List {
            r#type: r#type.to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

pub fn list_section_start<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    inside.push("list");
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = list_section_tag.context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    // let (source, mut children) = many0(alt((|src| {
    //     start_or_full_section(src, inside.clone())
    // }, list_item)))
    let (source, mut children) = many0(|src| list_item_with_sections(src, inside.clone()))
        .context("")
        .parse(source)?;
    let (source, end_section) = list_section_end(source, inside.clone(), r#type)?;
    children.push(end_section);
    Ok((
        source,
        Node::List {
            r#type: r#type.to_string(),
            children,
            bounds: "start".to_string(),
        },
    ))
}

pub fn list_section_tag<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, r#type) = alt((tag("list"),)).context("").parse(source)?;
    Ok((source, r#type))
}