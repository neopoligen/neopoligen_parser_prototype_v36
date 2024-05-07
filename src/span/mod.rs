// use crate::node::Node;
// use crate::section::*;
use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::character::complete::multispace0;
// use nom::multi::many0;
use nom::bytes::complete::is_not;
use nom::combinator::eof;
use nom::combinator::not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

#[derive(Debug)]
pub enum Span {
    WordPart { text: String },
}

pub fn span(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, span) = alt((word_part,)).context("").parse(source)?;
    Ok((source, span))
}

pub fn word_part(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    dbg!(&source);
    //let (source, _) = not(eof).context("").parse(source)?;
    let (source, word_part) = is_not(" \n").context("").parse(source)?;
    //
    dbg!(&source);

    Ok((
        source,
        Span::WordPart {
            text: word_part.to_string(),
        },
    ))
}
