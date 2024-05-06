use html_escape;
use neopoligen_parser_prototype_v36::*;
use std::fs;

fn main() {
    let content = r#"-- list

-/ a

asdf

// 

- b

"#;
    match parse(content) {
        Ok(ast) => {
            let out = output(&ast);
            let _ = fs::write(
                "output.html",
                format!(
                    "<!DOCTYPE html><html><head></head><body>{} <hr /><pre><code>{}<code></pre></body></html>",
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
