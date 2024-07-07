use rstest::rstest;
use assert_cmd::Command;
use predicates::prelude::*;

#[rstest]
#[case("version")]
#[case("-v")]
#[case("--version")]
fn test_version_command(#[case] command: &str) {
    Command::cargo_bin("workingon").unwrap()
    .args([
        command,
    ])
    .assert()
    .success()
    .stdout(
        predicate::str::starts_with(
            "workingon version",
        )
    );
}