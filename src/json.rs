use crate::basic::*;
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

#[allow(dead_code)]
pub fn json_section_end() {
    // TODO
}

pub fn json_section_full(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let kind = "json";
    // Note: this is not actually converted to json for the prototype, but
    // it will be in the full code
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = json_section_tag.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = many0(empty_until_newline_or_eof)
        .context("")
        .parse(source)?;
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

pub fn json_section_start<'a>(
    source: &'a str,
    mut inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let kind = "json";
    inside.push(kind);
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = json_section_tag.context("").parse(source)?;
    let end_key = format!("-- /{}", r#type);
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = empty_until_newline_or_eof.context("").parse(source)?;
    let (source, _) = many0(empty_until_newline_or_eof)
        .context("")
        .parse(source)?;
    let (source, data) = take_until(end_key.as_str()).context("").parse(source)?;
    let (source, _) = tag(end_key.as_str()).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Node::Json {
            kind: kind.to_string(),
            r#type: r#type.to_string(),
            data: data.trim_end().to_string(),
            bounds: "start".to_string(),
        },
    ))
}

pub fn json_section_tag<'a>(source: &'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, r#type) = alt((tag("metadata"), tag("metadata")))
        .context("")
        .parse(source)?;
    Ok((source, r#type))
}
