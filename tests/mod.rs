use neopoligen_parser_prototype_v36::*;
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case(
    "Basic Full Section",
    "-- div

alfa

",
    "<div><p>alfa</p></div>"
)]
#[case(
    "Basic Start/End Section",
    "-- div/

bravo

-- /div",
    "<div><p>bravo</p></div>"
)]
#[case(
    "Basic Full Inside Basic Start/End",
    "-- div/

charlie

-- div

delta

-- /div",
    "<div><p>charlie</p><div><p>delta</p></div></div>"
)]
#[case(
    "Basic Start/End Inside Basic Start/End",
    "-- div/

echo

-- div/

foxtrot

-- /div

golf

-- /div",
    "<div><p>echo</p><div><p>foxtrot</p></div><p>golf</p></div>"
)]
#[case(
    "List Full",
    "-- list

- alfa

- bravo

",
    "<ul><li><p>alfa</p></li><li><p>bravo</p></li></ul>"
)]
#[case(
    "List With Start/End Item",
    "-- list

-/ charlie

//

- delta

",
    "<ul><li><p>charlie</p></li><li><p>delta</p></li></ul>"
)]
#[case(
    "Basic Full Inside List Item Start/End",
    "-- list

-/ echo

-- div

foxtrot

//

- golf

",
    "<ul><li><p>echo</p><div><p>foxtrot</p></div></li><li><p>golf</p></li></ul>"
)]
#[case("List Full Inside List Item Start/End",
    "-- list

-/ hotel

-- list

- india

- juliet

//

- kilo

",
    "<ul><li><p>hotel</p><ul><li><p>india</p></li><li><p>juliet</p></li></ul></li><li><p>kilo</p></li></ul>"
)]
#[case(
    "Three levels of Basic Start/End",
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
#[case("Three Levels Of List Item Start/End",
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
#[case(
    "Raw Full",
    "-- pre

b",
    "<h2>pre</h2><pre>b</pre>"
)]
#[case(
    "Raw Keep Leading Whitespace",
    r#"-- pre

    c"#,
    "<h2>pre</h2><pre>    c</pre>"
)]
#[case(
    "Raw Start/End",
    r#"-- pre/

d

-- /pre"#,
    "<h2>pre</h2><pre>d</pre>"
)]
#[case(
    "Raw Start/End Inside List Item Start/End",
    r#"-- list

-/ a

-- pre/

b

-- /pre

//

- c

"#,
    "<ul><li><p>a</p><h2>pre</h2><pre>b</pre></li><li><p>c</p></li></ul>"
)]
#[case(
    "Checklist with start/end",
    r#"-- checklist

[]/ a

    -- pre/

b

    -- /pre

//

[] c

"#,
    "<ul><li><p>a</p><h2>pre</h2><pre>b</pre></li><li><p>c</p></li></ul>"
)]
#[case(
    "Generic Tag",
    r#"-- unknown-tag

a

"#,
    "<unknown-tag><p>a</p></unknown-tag>"
)]
#[case(
    "JSON Start/End",
    r#"-- metadata/

{}

-- /metadata

"#,
    "<h2>metadata</h2><pre>{}</pre>"
)]
fn run_tests(#[case] _x: &str, #[case] input: &str, #[case] left: &str) {
    let right = output(&parse(input).unwrap());
    assert_eq!(left, right);
}
