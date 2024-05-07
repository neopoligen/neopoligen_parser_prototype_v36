use crate::basic::*;
use crate::checklist::*;
use crate::list::*;
use crate::node::Node;
use crate::raw::*;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::combinator::eof;
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
    inside: Vec<&'a str>,
) -> IResult<&'a str, Node, ErrorTree<&'a str>> {
    let (source, results) = alt((
        |src| basic_section_full(src),
        |src| basic_section_start(src, inside.clone()),
        |src| checklist_section_full(src, inside.clone()),
        //        |src| json_section_full(src),
        //       |src| json_section_start(src, inside.clone()),
        |src| list_section_full(src, inside.clone()),
        |src| list_section_start(src, inside.clone()),
        |src| raw_section_full(src),
        |src| raw_section_start(src, inside.clone()),
        // make sure generic is last
        //      |src| generic_section_full(src),
        //     |src| generic_section_start(src, inside.clone()),
    ))
    .context("")
    .parse(source)?;
    Ok((source, results))
}
