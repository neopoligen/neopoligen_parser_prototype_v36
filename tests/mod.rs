use neopoligen_parser_prototype_v36::*;
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case("-- div\n\nalfa\n\n", "<div><p>alfa</p></div>")]
fn run_tests(#[case] input: &str, #[case] left: &str) {
    let right = output(&parse(input).unwrap());
    assert_eq!(left, right);
}
