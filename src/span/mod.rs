// use crate::node::Node;
// use crate::section::*;
use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::character::complete::multispace0;
// use nom::multi::many0;
use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
// use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
// use nom::combinator::eof;
// use nom::combinator::not;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

#[derive(Debug)]
pub enum Span {
    Newline { text: String },
    Space { text: String },
    WordPart { text: String },
}

pub fn span(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, span) = alt((newline, space, word_part)).context("").parse(source)?;
    Ok((source, span))
}

pub fn newline(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = tuple((space0, line_ending)).context("").parse(source)?;
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

pub fn word_part(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = is_not(" \n").context("").parse(source)?;
    Ok((
        source,
        Span::WordPart {
            text: text.to_string(),
        },
    ))
}
