use neopoligen_parser_prototype_v36::*;
use std::fs;

fn main() {
    let content = r#"-- list/

- 1

-- div/

alfa

-- /div

- 2

-- /list

"#;
    match parse(content) {
        Ok(ast) => {
            let out = output(&ast);
            let _ = fs::write(
                "output.html",
                format!(
                    "<!DOCTYPE html><html><head></head><body>{}</body></html>",
                    out
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
