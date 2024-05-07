use html_escape;
use neopoligen_parser_prototype_v36::*;
use std::fs;

fn main() {
    let sections = Sections {
        basic: vec!["div".to_string()],
        checklist: vec!["todo".to_string()],
        comment: vec!["comment".to_string()],
        detail: vec![],
        generic: vec![],
        json: vec!["json-example".to_string()],
        list: vec!["list".to_string()],
        raw: vec!["pre".to_string()],
        table: vec![],
        yaml: vec![],
    };

    let spans = vec!["em".to_string()];
    let content = r#"-- div

one ping only
    "#;
    match parse(content, &sections, &spans) {
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
