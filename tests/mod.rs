use neopoligen_parser_prototype_v36::*;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[test]
fn run_tests() {
    let dir = PathBuf::from("tests");
    get_files(&dir, vec!["txt"]).iter().for_each(|f| {
        let content = fs::read_to_string(f).unwrap();
        let tests = content
            .split("################################################")
            .collect::<Vec<&str>>();
        tests.iter().for_each(|t| {
            let parts = t
                .split("------------------------------------------------")
                .map(|p| p.trim_start())
                .collect::<Vec<&str>>();
            // if parts[0].starts_with("solo") && parts.len() == 3 {
            if !parts[0].starts_with("skip") && parts.len() == 3 {
                println!("{}", parts[0].trim());
                let left = parts[2].trim().replace("\n", "").replace(" ", "");
                let out = output(&parse(parts[1]).unwrap());
                let right = out.trim().replace("\n", "").replace(" ", "");
                assert_eq!(left, right);
            }
        });
    });
}

fn get_files(dir: &PathBuf, exts: Vec<&str>) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter(|e| match e.as_ref().unwrap().path().extension() {
            Some(x) => exts.contains(&x.to_str().unwrap()),
            None => false,
        })
        .map(|e| e.unwrap().into_path())
        .collect()
}