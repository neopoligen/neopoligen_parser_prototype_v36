pub mod basic;
pub mod checklist;
pub mod comment;
pub mod generic;
pub mod json;
pub mod list;
pub mod node;
pub mod raw;
pub mod yaml;

use crate::section::basic::*;
use crate::section::checklist::*;
use crate::section::comment::*;
use crate::section::generic::*;
use crate::section::json::*;
use crate::section::list::*;
use crate::node::Node;
use crate::raw::*;
use crate::yaml::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn block_of_anything(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let (source, _) = not(tag("--")).context("").parse(source)?;
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

pub fn block_of_end_content(source: &str) -> IResult<&str, Node, ErrorTree<&str>> {
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, _) = not(tag("[")).context("").parse(source)?;
    // let (source, _) = not(tag("//")).context("").parse(source)?;
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

pub fn empty_until_newline_or_eof<'a>(
    source: &'a str,
) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, _) = alt((
        tuple((space0, newline.map(|_| ""))),
        tuple((multispace0, eof.map(|_| ""))),
    ))
    .context("")
    .parse(source)?;
    Ok((source, ""))
}

pub fn start_or_full_section<'a>(
    source: &'a str,
    inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, results) = alt((
        |src| basic_section_full(src),
        |src| basic_section_start(src, inside.clone()),
        |src| checklist_section_full(src, inside.clone()),
        |src| checklist_section_start(src, inside.clone()),
        |src| comment_section_full(src),
        |src| comment_section_start(src, inside.clone()),
        |src| json_section_full(src),
        |src| json_section_start(src, inside.clone()),
        |src| list_section_full(src, inside.clone()),
        |src| list_section_start(src, inside.clone()),
        |src| raw_section_full(src),
        |src| raw_section_start(src, inside.clone()),
        |src| yaml_section_full(src),
        |src| yaml_section_start(src, inside.clone()),
        // make sure generic is last
        |src| generic_section_full(src),
        |src| generic_section_start(src, inside.clone()),
    ))
    .context("")
    .parse(source)?;
    Ok((source, results))
}
