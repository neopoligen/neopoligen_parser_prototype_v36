pub mod basic;
pub mod checklist;
pub mod comment;
pub mod generic;
pub mod json;
pub mod list;
pub mod node;
pub mod raw;
pub mod yaml;

use crate::node::Node;
use crate::raw::*;
use crate::section::basic::*;
use crate::section::checklist::*;
use crate::section::comment::*;
use crate::section::generic::*;
use crate::section::json::*;
use crate::section::list::*;
use crate::span::*;
use crate::yaml::*;
use crate::Sections;
use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::combinator::eof;
// use nom::combinator::not;
// use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

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
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, results) = alt((
        |src| basic_section_full(src, &sections, &spans),
        |src| basic_section_start(src, &sections, &spans),
        |src| checklist_section_full(src, &sections, &spans),
        |src| checklist_section_start(src, &sections, &spans),
        |src| comment_section_full(src, &sections, &spans),
        |src| comment_section_start(src, &sections, &spans),
        |src| json_section_full(src, &sections, &spans),
        |src| json_section_start(src, &sections, &spans),
        |src| list_section_full(src, &sections, &spans),
        |src| list_section_start(src, &sections, &spans),
        |src| raw_section_full(src, &sections, &spans),
        |src| raw_section_start(src, &sections, &spans),
        |src| yaml_section_full(src, &sections, &spans),
        |src| yaml_section_start(src, &sections, &spans),
        // make sure generic is last
        |src| generic_section_full(src, &sections, &spans),
        |src| generic_section_start(src, &sections, &spans),
    ))
    .context("")
    .parse(source)?;
    Ok((source, results))
}
