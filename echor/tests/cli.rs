use std::fs;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;

type TestResult = Result<()>;

fn echor() -> Result<Command, assert_cmd::cargo::CargoError> {
    Command::cargo_bin("echor")
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;

    let output = echor()?.args(args).output().expect("fail");
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    echor()?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    echor()?.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
