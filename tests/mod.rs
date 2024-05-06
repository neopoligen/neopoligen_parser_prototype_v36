use neopoligen_parser_prototype_v36::*;
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case("-- div\n\nalfa\n\n", "<div><p>alfa</p></div>")]
#[case("-- div/\n\nbravo\n\n-- /div\n\n", "<div><p>bravo</p></div>")]
#[case(
    "-- div/\n\ncharlie\n\n-- div\n\ndelta\n\n-- /div\n\n",
    "<div><p>charlie</p><div><p>delta</p></div></div>"
)]
#[case(
    "-- div/\n\necho\n\n-- div/\n\nfoxtrot\n\n-- /div\n\ngolf\n\n-- /div\n\n",
    "<div><p>echo</p><div><p>foxtrot</p></div><p>golf</p></div>"
)]
fn run_tests(#[case] input: &str, #[case] left: &str) {
    let right = output(&parse(input).unwrap());
    assert_eq!(left, right);
}
