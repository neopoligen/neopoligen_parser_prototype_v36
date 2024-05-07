use crate::block::*;
use crate::node::Node;
use crate::section::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn yaml_section_end<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>,
    key: &'a str,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    inside.pop();
    let kind = "yaml";
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, r#type) = tag(key).context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(block_of_end_content).context("").parse(source)?;
    Ok((
        source,
        Node::Yaml {
            bounds: "end".to_string(),
            children,
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            data: None,
        },
    ))
}

pub fn yaml_section_full(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let kind = "yaml";
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = yaml_section_tag.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = many0(empty_until_newline_or_eof)
        .context("")
        .parse(source)?;
    let (source, text) = alt((take_until("\n--"), rest)).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Yaml {
            bounds: "full".to_string(),
            children: vec![],
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            data: Some(text.trim_end().to_string()),
        },
    ))
}

pub fn yaml_section_start<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "yaml";
    inside.push(kind);
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = yaml_section_tag.context("").parse(source)?;
    let end_key = format!("-- /{}", r#type);
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = many0(empty_until_newline_or_eof)
        .context("")
        .parse(source)?;
    let (source, text) = take_until(end_key.as_str()).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, end_section) = yaml_section_end(source, inside.clone(), r#type)?;
    Ok((
        source,
        Node::Yaml {
            bounds: "start".to_string(),
            children: vec![end_section],
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            data: Some(text.trim_end().to_string()),
        },
    ))
}

pub fn yaml_section_tag<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, r#type) = alt((tag("yaml-example"),)).context("").parse(source)?;
    Ok((source, r#type))
}
