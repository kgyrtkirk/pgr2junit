#![allow(unused)] // FIXME
use patch::Patch;
use scanf::sscanf;
use std::{
    collections::HashMap,
    fs, io,
    iter::Map,
    path::{Path, PathBuf},
    result,
};

#[derive(Debug)]
pub struct TestCase {
    pub name: String,
    pub result: Result,
    pub diff: String,
}

#[derive(Debug)]
pub struct Model {
    pub cases: Vec<TestCase>,
}

#[derive(Debug, PartialEq)]
pub enum State {
    PASSED,
    FAILED,
    IGNORED,
}
#[derive(Debug)]
pub struct Result {
    pub state: State,
    pub runtime: f32,
}

impl Model {
    pub fn new(path: &Path) -> Model {
        let results = read_results(read_file(path, "regression.out"));
        let diffs = read_diffs(read_file(path, "regression.diffs"));

        println!("{:?}", diffs);
        let mut cases: Vec<TestCase> = Vec::new();

        for (name, result) in results {
            cases.push(TestCase {
                result: result,
                diff: diffs.get(&name).unwrap_or(&String::new()).to_owned(),
                name: name,
            });
        }

        return Model { cases };
    }
}
fn read_file(path: &Path, file: &str) -> String {
    let mut file_path = path.to_path_buf();
    file_path.push(file);
    fs::read_to_string(file_path).unwrap()
}

fn read_results(results: String) -> HashMap<String, Result> {
    let mut res = HashMap::new();

    for line in results.split_terminator("\n") {
        let (test_name, result) = parse_result_line(line);
        res.insert(test_name, result);
    }
    res
}

fn parse_result_line(line: &str) -> (String, Result) {
    let mut name: String = Default::default();
    let mut state: String = Default::default();
    let mut time: f32 = Default::default();

    let line = line.replace("failed (ignored)", "ignored");
    //test misc                         ... FAILED      283 ms
    sscanf!(line.as_str(), "test {} ... {} {} ms", name, state, time);

    println!(">{:?}< ", state);
    let state = match state.as_str() {
        "FAILED" => State::FAILED,
        "ok" => State::PASSED,
        "ignored" => State::IGNORED,
        _ => todo!("unhandled state >{:?}<?!", state),
    };

    (
        name,
        Result {
            runtime: time,
            state: state,
        },
    )
}

fn read_diffs(diffs: String) -> HashMap<String, String> {
    let patches = Patch::from_multiple(&diffs).unwrap();
    let mut res = HashMap::new();
    for patch in patches {
        let path = patch.old.path.to_string();
        let name = Path::new(&path).with_extension("");
        let name = name.file_name().unwrap();
        let name = String::from(name.to_str().unwrap());
        res.insert(name, patch.to_string());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_result_line_failed() {
        let (name, res) =
            parse_result_line("test misc                         ... FAILED      283 ms");
        assert_eq!(name, "misc");
        assert_eq!(res.state, State::FAILED);
        assert_eq!(res.runtime, 283.0);
    }
    #[test]
    fn test_parse_result_line_ok() {
        let (name, res) = parse_result_line("test  misc  ... ok      283.3 ms");
        assert_eq!(name, "misc");
        assert_eq!(res.state, State::PASSED);
        assert_eq!(res.runtime, 283.3);
    }
    #[test]
    fn test_parse_result_line_ignore() {
        let (name, res) = parse_result_line("test  misc  ... failed (ignored)      283.3 ms");
        assert_eq!(name, "misc");
        assert_eq!(res.state, State::IGNORED);
        assert_eq!(res.runtime, 283.3);
    }
}
