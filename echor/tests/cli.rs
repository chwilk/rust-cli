use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn works () {
    assert!(true);
}

#[test]
fn runs_without_args () {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\n"));
}

#[test]
fn echo_echoes() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}