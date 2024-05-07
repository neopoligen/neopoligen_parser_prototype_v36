use html_escape;
use neopoligen_parser_prototype_v36::*;
use std::fs;

fn main() {
    let content = r#"-- list/

- hotel

    -- list/

    - india

        -- list/

        - asdf

        - awerwe

        -- /list

    - juliet

    -- /list

- kilo

-- /list


    "#;
    match parse(content) {
        Ok(ast) => {
            let out = output(&ast);
            let _ = fs::write(
                "output.html",
                format!(
                    r#"<!DOCTYPE html>
<html>
    <head>
        <style>
            body {{ background-color: #444; color: #aaa; }}
            div {{ border: 1px solid black; padding: 0.2rem; margin: 0.4rem;}}
        </style>
    </head>
    <body>
        {}
        <div>
        <pre><code>{}<code></pre>
        </div>
    </body>
</html>"#,
                    out,
                    html_escape::encode_text(&out)
                ),
            );
        }
        Err(e) => {
            println!(
                "ERROR\nLine: {}\nColumn: {}\nMessage: {}\nRemainder:\n{}",
                e.line, e.column, e.message, e.remainder,
            );
        }
    };
}
