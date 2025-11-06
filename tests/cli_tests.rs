use assert_cmd::Command;
use diesel::prelude::*;
use predicates::prelude::*;
use rstest::rstest;
use serial_test::serial;
use tempdir::TempDir;
use workingon::schema::todos::dsl::*;
use workingon::{encode_id, establish_connection};

// Helper function to get the latest TODO from the database
fn get_latest_todo() -> Option<(String, workingon::models::Todos)> {
    let connection = &mut establish_connection();
    let results = todos
        .select(workingon::models::Todos::as_select())
        .order_by(id.desc())
        .limit(1)
        .load(connection)
        .expect("Error loading todos");

    if results.is_empty() {
        None
    } else {
        let todo = results.into_iter().next().unwrap();
        let encoded_id = encode_id(todo.id.try_into().unwrap());
        Some((encoded_id, todo))
    }
}

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

#[rstest]
fn test_help_command() {
    Command::cargo_bin("workingon")
        .unwrap()
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "CLI to track what you\'re working on",
        ));
}

#[test]
fn test_list_empty() {
    let tmp_dir = TempDir::new("workingon_test").expect("cannot make temp directory for test");
    Command::cargo_bin("workingon")
        .unwrap()
        .env("WORKINGON_DATA_DIR", tmp_dir.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_list_alias_ls() {
    let tmp_dir = TempDir::new("workingon_test").expect("cannot make temp directory for test");
    Command::cargo_bin("workingon")
        .unwrap()
        .env("WORKINGON_DATA_DIR", tmp_dir.path())
        .args(["ls"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_add_todo_with_title() {
    let tmp_dir = TempDir::new("workingon_test").expect("cannot make temp directory for test");
    // Set a mock editor to avoid interactive prompts
    Command::cargo_bin("workingon")
        .unwrap()
        .env("EDITOR", "-")
        .env("WORKINGON_DATA_DIR", tmp_dir.path())
        .args(["add", "Test TODO"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TODO added successfully"));
}

#[test]
fn test_add_todo_without_title() {
    let tmp_dir = TempDir::new("workingon_test").expect("cannot make temp directory for test");
    // Set a mock editor to avoid interactive prompts
    let mut cmd = Command::cargo_bin("workingon").unwrap();
    cmd.env("EDITOR", "-");
    cmd.env("WORKINGON_DATA_DIR", tmp_dir.path());

    cmd.args(["add"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TODO added successfully"));
}

#[test]
fn test_add_and_list_todos() {
    let tmp_dir = TempDir::new("workingon_test").expect("cannot make temp directory for test");
    // Add a TODO
    let mut cmd = Command::cargo_bin("workingon").unwrap();
    cmd.env("EDITOR", "-");
    cmd.env("WORKINGON_DATA_DIR", tmp_dir.path());

    cmd.args(["add", "First TODO"]).assert().success();

    // Add another TODO
    Command::cargo_bin("workingon")
        .unwrap()
        .env("EDITOR", "-")
        .env("WORKINGON_DATA_DIR", tmp_dir.path())
        .args(["add", "Second TODO"])
        .assert()
        .success();

    // List TODOs
    Command::cargo_bin("workingon")
        .unwrap()
        .env("EDITOR", "-")
        .env("WORKINGON_DATA_DIR", tmp_dir.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("First TODO"))
        .stdout(predicate::str::contains("Second TODO"));
}

#[test]
#[serial]
fn test_add_and_show_todo() {
    let tmp_dir = TempDir::new("workingon_test").expect("cannot make temp directory for test");

    // Set environment variable for the test
    std::env::set_var("WORKINGON_DATA_DIR", tmp_dir.path().to_string_lossy().to_string());
    std::env::set_var("EDITOR", "-");

    // Add a TODO using the library
    workingon::add_todo(Some("First TODO".to_string()));

    // Get the TODO ID directly from the database
    let (todo_id, _todo) = get_latest_todo().expect("No todo found");

    println!("{}", todo_id);

    // Verify the TODO was added by listing
    Command::cargo_bin("workingon")
        .unwrap()
        .env("EDITOR", "-")
        .env("WORKINGON_DATA_DIR", tmp_dir.path())
        .args(["ls"])
        .assert()
        .success()
        .stdout(predicate::str::contains("First TODO"));

    // Show the TODO using the library
    workingon::show_todo(todo_id);
}

#[test]
#[serial]
fn test_complete_and_reopen_todo() {
    let tmp_dir = TempDir::new("workingon_test").expect("cannot make temp directory for test");

    // Set environment variable for the test
    std::env::set_var("WORKINGON_DATA_DIR", tmp_dir.path().to_string_lossy().to_string());
    std::env::set_var("EDITOR", "-");

    // Add a TODO using the library
    workingon::add_todo(Some("Complete and Reopen Test TODO".to_string()));

    // Get the TODO ID directly from the database
    let (todo_id, todo) = get_latest_todo().expect("No todo found");

    // Verify it's not completed initially
    assert!(todo.completed.is_none());

    // Complete the TODO via CLI
    Command::cargo_bin("workingon")
        .unwrap()
        .env("EDITOR", "-")
        .env("WORKINGON_DATA_DIR", tmp_dir.path())
        .args(["complete", &todo_id])
        .assert()
        .success()
        .stdout(predicate::str::contains("TODO was completed"));

    // Verify it's completed by checking the database
    let connection = &mut workingon::establish_connection();
    let completed_results = todos
        .select(workingon::models::Todos::as_select())
        .filter(id.eq(todo.id))
        .load(connection)
        .expect("Error loading todos");

    assert_eq!(completed_results.len(), 1);
    assert!(completed_results[0].completed.is_some());

    // Reopen the TODO via CLI
    Command::cargo_bin("workingon")
        .unwrap()
        .env("EDITOR", "-")
        .env("WORKINGON_DATA_DIR", tmp_dir.path())
        .args(["reopen", &todo_id])
        .assert()
        .success()
        .stdout(predicate::str::contains("TODO was reopened"));

    // Verify it's reopened by checking the database
    let reopened_results = todos
        .select(workingon::models::Todos::as_select())
        .filter(id.eq(todo.id))
        .load(connection)
        .expect("Error loading todos");

    assert_eq!(reopened_results.len(), 1);
    assert!(reopened_results[0].completed.is_none());
}
