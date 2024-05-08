use crate::node::Node;
// use crate::raw::*;
// use crate::section::basic::*;
// use crate::section::checklist::*;
// use crate::section::comment::*;
// use crate::section::generic::*;
// use crate::section::json::*;
// use crate::section::list::*;
use crate::span::*;
// use crate::yaml::*;
// use nom::branch::alt;
use nom::bytes::complete::tag;
// use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
// use nom::character::complete::newline;
// use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many0;
// use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn block_of_anything<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("--")).context("").parse(source)?;
    let (source, spans) = many0(|src| span_finder(src, spans))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((source, Node::Block { spans }))
}

pub fn block_of_end_content<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, _) = not(tag("[")).context("").parse(source)?;
    let (source, spans) = many0(|src| span_finder(src, spans))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((source, Node::Block { spans }))
}
