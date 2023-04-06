#![allow(unused)] // FIXME
use patch::Patch;
use scanf::sscanf;
use std::{
    collections::HashMap,
    fs::{self, File},
    io,
    iter::Map,
    path::{Path, PathBuf},
    result,
    time::Duration,
};

#[derive(Debug)]
pub struct TestCase {
    pub name: String,
    pub result: Result,
    pub diff: String,
}

#[derive(Debug)]
pub struct Model {
    pub path: PathBuf,
    pub name: String,
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
    pub runtime: Duration,
}

impl Model {
    pub fn new(path: &Path) -> Model {
        let results = read_results(read_file(path, "regression.out"));
        let diffs = read_diffs(read_file(path, "regression.diffs"));

        let mut cases: Vec<TestCase> = Vec::new();

        for (name, result) in results {
            cases.push(TestCase {
                result: result,
                diff: diffs.get(&name).unwrap_or(&String::new()).to_owned(),
                name: name,
            });
        }

        let suite_name = String::from(path.to_str().unwrap());

        return Model {
            path: path.to_path_buf(),
            name: suite_name,
            cases,
        };
    }

    pub fn save(&self) {
        let mut out_path = self.path.clone();
        out_path.push("regression.xml");
        let mut file = File::create(out_path).unwrap();
        use std::io::prelude::*;
        file.write_all(self.to_junit().as_bytes());
    }
    pub fn to_junit(&self) -> String {
        use quick_junit::*;

        let mut report = Report::new("my-test-run");
        let mut test_suite = TestSuite::new(&self.name);
        for case in &self.cases {
            let mut status = match case.result.state {
                State::IGNORED => TestCaseStatus::skipped(),
                State::PASSED => TestCaseStatus::success(),
                State::FAILED => TestCaseStatus::non_success(NonSuccessKind::Failure),
            };
            status.set_description(case.diff.clone());

            TestCaseStatus::success();
            let mut out_case = TestCase::new(&case.name, status);
            out_case.set_time(case.result.runtime);
            out_case.set_system_out("asd");

            test_suite.add_test_case(out_case);
        }
        // let success_case = TestCase::new("success-case", TestCaseStatus::success());
        // let failure_case = TestCase::new(
        //     "failure-case",
        //     TestCaseStatus::non_success(NonSuccessKind::Failure),
        // );
        // test_suite.add_test_cases([success_case, failure_case]);
        report.add_test_suite(test_suite);

        return report.to_string().unwrap();
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
    let mut time: f64 = Default::default();

    let line = line.replace("failed (ignored)", "ignored");
    //test misc                         ... FAILED      283 ms
    sscanf!(line.as_str(), "test {} ... {} {} ms", name, state, time);

    let state = match state.as_str() {
        "FAILED" => State::FAILED,
        "ok" => State::PASSED,
        "ignored" => State::IGNORED,
        _ => todo!("unhandled state >{:?}<?!", state),
    };

    (
        name,
        Result {
            runtime: Duration::from_secs_f64(time / 1000.0),
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
        assert_eq!(res.runtime.as_secs_f32(), 0.283);
    }
    #[test]
    fn test_parse_result_line_ok() {
        let (name, res) = parse_result_line("test  misc  ... ok      283.3 ms");
        assert_eq!(name, "misc");
        assert_eq!(res.state, State::PASSED);
        assert_eq!(res.runtime.as_secs_f32(), 0.2833);
    }
    #[test]
    fn test_parse_result_line_ignore() {
        let (name, res) = parse_result_line("test  misc  ... failed (ignored)      283.3 ms");
        assert_eq!(name, "misc");
        assert_eq!(res.state, State::IGNORED);
        assert_eq!(res.runtime.as_secs_f32(), 0.2833);
    }
}
