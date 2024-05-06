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
#[case(
    "-- list\n\n- alfa\n\n- bravo\n\n",
    "<ul><li><p>alfa</p></li><li><p>bravo</p></li></ul>"
)]
#[case(
    "-- list\n\n-/ charlie\n\n//\n\n- delta\n\n",
    "<ul><li><p>charlie</p></li><li><p>delta</p></li></ul>"
)]
#[case(
    "-- list\n\n-/ echo\n\n-- div\n\nfoxtrot\n\n//\n\n- golf\n\n",
    "<ul><li><p>echo</p><div><p>foxtrot</p></div></li><li><p>golf</p></li></ul>"
)]
#[case(
    "-- list\n\n-/ hotel\n\n-- list\n\n- india\n\n- juliet\n\n//\n\n- kilo\n\n",
    "<ul><li><p>hotel</p><ul><li><p>india</p></li><li><p>juliet</p></li></ul></li><li><p>kilo</p></li></ul>"
)]
#[case(
    r#"-- div/

a

    -- div/

    b

        -- div/

        c

        -- /div

    d

    -- /div

e

-- /div

"#,
    "<div><p>a</p><div><p>b</p><div><p>c</p></div><p>d</p></div><p>e</p></div>"
)]
#[case(
    r#"-- list

-/ a

    -- list

    - c

    -/ d

        -- list

        - e

    //

    -- div

    here

//

- b

f

"#,
    "<ul><li><p>a</p><ul><li><p>c</p></li><li><p>d</p><ul><li><p>e</p></li></ul></li></ul><div><p>here</p></div></li><li><p>b</p><p>f</p></li></ul>"
)]
#[case("-- pre\n\na\n\n-- div\n\nb\n\n", "<pre>a</pre><div><p>b</p></div>")]
#[case("-- pre\n\nb", "<pre>b</pre>")]
#[case("-- pre\n\n\n\n    c", "<pre>    c</pre>")]
#[case("-- pre/\n\nd\n\n-- /pre", "<pre>d</pre>")]
fn run_tests(#[case] input: &str, #[case] left: &str) {
    let right = output(&parse(input).unwrap());
    assert_eq!(left, right);
}
