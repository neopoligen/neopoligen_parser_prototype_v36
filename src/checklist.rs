use crate::basic::*;
use crate::node::Node;
use crate::section::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::not;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn checklist_item_block(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let (source, _) = not(tag("[")).context("").parse(source)?;
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

pub fn checklist_item(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    // NOTE: this prototype doesn't not distinguish between checked
    // and unchecked. Everything is targeted to unchecked
    let (source, _) = tag("[]").context("").parse(source)?;
    let (source, children) = many0(checklist_item_block).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::ChecklistItem {
            children,
            status: false,
            status_value: None,
        },
    ))
}

pub fn checklist_item_with_sections<'a>(
    source: &'a str,
    inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, _) = tag("[] ").context("").parse(source)?;
    let (source, children) = many0(alt((checklist_item_block, |src| {
        start_or_full_section(src, inside.clone())
    })))
    .context("")
    .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::ChecklistItem {
            children,
            status: false,
            status_value: None,
        },
    ))
}

pub fn checklist_section_end<'a>(
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
    let (source, children) = many0(basic_block_not_list_item).context("").parse(source)?;
    Ok((
        source,
        Node::Checklist {
            r#type: r#type.to_string(),
            children,
            bounds: "end".to_string(),
        },
    ))
}

pub fn checklist_section_full<'a>(
    source: &'a str,
    mut _inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = checklist_section_tag.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(checklist_item).context("").parse(source)?;
    Ok((
        source,
        Node::Checklist {
            r#type: r#type.to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

pub fn checklist_section_start<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    inside.push("list");
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = checklist_section_tag.context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, mut children) = many0(|src| checklist_item_with_sections(src, inside.clone()))
        .context("")
        .parse(source)?;
    let (source, end_section) = checklist_section_end(source, inside.clone(), r#type)?;
    children.push(end_section);
    Ok((
        source,
        Node::Checklist {
            r#type: r#type.to_string(),
            children,
            bounds: "start".to_string(),
        },
    ))
}

pub fn checklist_section_tag<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, r#type) = alt((tag("todo"),)).context("").parse(source)?;
    Ok((source, r#type))
}
