use crate::block::*;
use crate::node::Node;
use crate::section::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn basic_section_end<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
    key: &'a str,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, r#type) = tag(key).context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(|src| block_of_end_content(src, spans))
        .context("")
        .parse(source)?;
    Ok((
        source,
        Node::Basic {
            r#type: r#type.to_string(),
            children,
            bounds: "end".to_string(),
        },
    ))
}

pub fn basic_section_full<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.basic))
        .context("")
        .parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(|src| block_of_anything(src, &spans))
        .context("")
        .parse(source)?;
    Ok((
        source,
        Node::Basic {
            r#type: r#type.to_string(),
            children,
            bounds: "full".to_string(),
        },
    ))
}

pub fn basic_section_start<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.basic))
        .context("")
        .parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, mut children) = many0(alt((
        |src| block_of_anything(src, &spans),
        |src| start_or_full_section(src, &sections, &spans),
    )))
    .context("")
    .parse(source)?;
    let (source, end_section) = basic_section_end(source, &spans, r#type)?;
    children.push(end_section);
    Ok((
        source,
        Node::Basic {
            r#type: r#type.to_string(),
            children,
            bounds: "start".to_string(),
        },
    ))
}
