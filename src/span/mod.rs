// use crate::node::Node;
// use crate::section::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
// use nom::character::complete::multispace0;
use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::multi::many0;
// use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
// use nom::combinator::eof;
use nom::combinator::not;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Span {
    KnownSpan {
        r#type: String,
        spans: Vec<Span>,
        flags: Vec<String>,
        attrs: BTreeMap<String, String>,
    },
    Newline {
        text: String,
    },
    Space {
        text: String,
    },
    WordPart {
        text: String,
    },
}

pub enum SpanAttr {
    KeyValue { key: String, value: String },
    Flat { key: String },
}

pub fn span_finder(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, span) = alt((known_span, newline, space, word_part))
        .context("")
        .parse(source)?;
    Ok((source, span))
}

pub fn newline(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = tuple((space0, line_ending)).context("").parse(source)?;
    let (source, _) = not(tuple((space0, line_ending)))
        .context("")
        .parse(source)?;
    Ok((
        source,
        Span::Space {
            text: text.1.to_string(),
        },
    ))
}

pub fn space(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = space1.context("").parse(source)?;
    Ok((
        source,
        Span::Space {
            text: text.to_string(),
        },
    ))
}

pub fn known_span(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag("<<").context("").parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    let (source, r#type) = known_span_type.context("").parse(source)?;
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, spans) = many0(span_finder).context("").parse(source)?;
    let (source, raw_attrs) = many0(span_attr).context("").parse(source)?;
    let (source, _) = tag(">>").context("").parse(source)?;

    let mut flags: Vec<String> = vec![];
    let mut attrs = BTreeMap::new();
    raw_attrs.iter().for_each(|attr| match attr {
        SpanAttr::KeyValue { key, value } => {
            attrs.insert(key.to_string(), value.to_string());
        }
        SpanAttr::Flat { key } => flags.push(key.to_string()),
    });
    Ok((
        source,
        Span::KnownSpan {
            r#type: r#type.to_string(),
            spans,
            flags,
            attrs,
        },
    ))
}

pub fn span_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, key) = is_not(" |\n:").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, value) = is_not(">|").context("").parse(source)?;
    Ok((
        source,
        SpanAttr::KeyValue {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        },
    ))
}

pub fn word_part(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = is_not(" \n<>|").context("").parse(source)?;
    Ok((
        source,
        Span::WordPart {
            text: text.to_string(),
        },
    ))
}

pub fn known_span_type(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, r#type) = alt((tag("em"), tag("strong"))).context("").parse(source)?;
    Ok((source, r#type))
}
