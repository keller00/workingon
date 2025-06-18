use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;

#[rstest]
#[case("version")]
#[case("-v")]
#[case("--version")]
fn test_version_command(#[case] command: &str) {
    Command::cargo_bin("workingon")
        .unwrap()
        .args([command])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("workingon version"));
}

#[rstest]
fn test_locate_db() {
    Command::cargo_bin("workingon")
        .unwrap()
        .args(["locate-db"])
        .assert()
        .success()
        .stdout(predicate::str::ends_with("todos.sqlite3\n"));
}
