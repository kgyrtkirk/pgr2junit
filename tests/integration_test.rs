#![allow(unused)] // FIXME
use std::{fs, path::Path};

use pgr2junit::pgr_model::{Model, State};

#[test]
fn fail() {
    let model = Model::new(Path::new("data/fail"));
    assert_eq!(model.cases.len(), 1);
    let c = model.cases.get(0).unwrap();
    assert_eq!(c.result.state, State::FAILED);
    assert_eq!(c.result.runtime, 283.0);
    assert_eq!(c.name, "misc");
    assert_eq!(c.diff.len(), 528);
}

#[test]
fn mixed() {
    let model = Model::new(Path::new("data/mixed"));
    assert_eq!(model.cases.len(), 3);
}
