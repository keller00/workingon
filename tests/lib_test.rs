use diesel::prelude::*;
use serial_test::serial;
use std::env;
use tempdir::TempDir;
use workingon::*;

// Helper function to set up a test environment
fn setup_test_env() -> TempDir {
    let tmp_dir = TempDir::new("workingon_test").expect("cannot make temp directory for test");
    env::set_var("WORKINGON_DATA_DIR", tmp_dir.path());
    env::set_var("EDITOR", "-");
    tmp_dir
}

// Helper function to clean up test environment
fn cleanup_test_env() {
    env::remove_var("WORKINGON_DATA_DIR");
    env::remove_var("EDITOR");
}

#[test]
#[serial]
fn test_encode_id() {
    let encoded = encode_id(1);
    assert!(!encoded.is_empty());
    assert!(encoded.len() >= 5); // min_length is 5
    assert!(encoded
        .chars()
        .all(|c| "1234567890abcdefghijklmnopqrstuvwxyz".contains(c)));
}

#[test]
#[serial]
fn test_decode_id() {
    let original_id = 42;
    let encoded = encode_id(original_id);
    let decoded = decode_id(&encoded);
    assert_eq!(decoded, original_id as i32);
}

#[test]
#[serial]
fn test_encode_decode_roundtrip() {
    let test_cases = vec![1, 100, 1000, 9999];
    for id in test_cases {
        let encoded = encode_id(id);
        let decoded = decode_id(&encoded);
        assert_eq!(decoded, id as i32);
    }
}

#[test]
#[serial]
fn test_get_project_data_folder_with_env_var() {
    let _tmp_dir = TempDir::new("workingon_test").expect("cannot make temp directory for test");
    let env_var_name = format!("{}_data_dir", workingon::constants::BIN).to_uppercase();
    env::set_var(&env_var_name, _tmp_dir.path());

    let result = get_project_data_folder();
    assert_eq!(result, _tmp_dir.path());

    env::remove_var(&env_var_name);
}

#[test]
#[serial]
fn test_get_project_data_folder_without_env_var() {
    // Clean up any existing env var
    let env_var_name = format!("{}_data_dir", workingon::constants::BIN).to_uppercase();
    env::remove_var(&env_var_name);

    let result = get_project_data_folder();
    assert!(result.exists());
    assert!(result.is_dir());
    assert!(result.to_string_lossy().contains(workingon::constants::BIN));
}

#[test]
#[serial]
fn test_get_db_file() {
    let _tmp_dir = setup_test_env();
    let db_file = get_db_file();

    assert!(db_file.to_string_lossy().ends_with("todos.sqlite3"));
    assert_eq!(db_file.parent().unwrap(), _tmp_dir.path());

    cleanup_test_env();
}

#[test]
#[serial]
fn test_get_todoeditmsg_file() {
    let _tmp_dir = setup_test_env();
    let todo_file = get_todoeditmsg_file();

    assert!(todo_file.to_string_lossy().ends_with("TODO_EDITMSG"));
    assert_eq!(todo_file.parent().unwrap(), _tmp_dir.path());

    cleanup_test_env();
}

#[test]
#[serial]
fn test_get_editor_with_env_var() {
    env::set_var("EDITOR", "nano");
    let editor = get_editor();
    assert_eq!(editor, "nano");
    env::remove_var("EDITOR");
}

#[test]
#[serial]
fn test_get_editor_without_env_var() {
    env::remove_var("EDITOR");
    let editor = get_editor();
    assert_eq!(editor, workingon::constants::DEFAULT_EDITOR);
}

#[test]
#[serial]
fn test_establish_connection() {
    let _tmp_dir = setup_test_env();

    let _connection = establish_connection();
    // If we get here without panicking, the connection was established successfully
    assert!(true);

    cleanup_test_env();
}

#[test]
#[serial]
fn test_create_temp_todo_file_with_editor_dash() {
    let _tmp_dir = setup_test_env();
    let test_file = _tmp_dir.path().join("test_todo.txt");

    let (title, notes) = create_temp_todo_file_open_and_then_read_remove_process(
        &test_file,
        "Test Title".to_string(),
        "Test Notes".to_string(),
    );

    assert_eq!(title, "Test Title");
    assert_eq!(notes, "Test Notes");
    assert!(!test_file.exists()); // File should be removed

    cleanup_test_env();
}

#[test]
#[serial]
fn test_create_temp_todo_file_with_real_editor() {
    let _tmp_dir = setup_test_env();
    let test_file = _tmp_dir.path().join("test_todo.txt");

    // Set a real editor that exists
    env::set_var("EDITOR", "echo");

    let (title, notes) = create_temp_todo_file_open_and_then_read_remove_process(
        &test_file,
        "Test Title".to_string(),
        "Test Notes".to_string(),
    );

    // With echo as editor, the file content should be processed
    assert_eq!(title, "Test Title");
    assert_eq!(notes, "Test Notes");
    assert!(!test_file.exists()); // File should be removed

    cleanup_test_env();
}

#[test]
#[serial]
fn test_add_todo_with_title() {
    let _tmp_dir = setup_test_env();
    env::set_var("EDITOR", "-"); // Ensure editor is set to dash

    add_todo(Some("Test TODO".to_string()));

    // Verify the TODO was added by checking the database
    let connection = &mut establish_connection();
    use workingon::schema::todos::dsl::*;
    let results = todos
        .select(workingon::models::Todos::as_select())
        .filter(title.eq("Test TODO"))
        .load(connection)
        .expect("Error loading todos");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Test TODO");

    cleanup_test_env();
}

#[test]
#[serial]
fn test_add_todo_without_title() {
    let _tmp_dir = setup_test_env();
    env::set_var("EDITOR", "-"); // Ensure editor is set to dash

    add_todo(None);

    // Verify the TODO was added with default title
    let connection = &mut establish_connection();
    use workingon::schema::todos::dsl::*;
    let results = todos
        .select(workingon::models::Todos::as_select())
        .filter(title.eq("<title>"))
        .load(connection)
        .expect("Error loading todos");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "<title>");

    cleanup_test_env();
}

#[test]
#[serial]
fn test_show_todo() {
    let _tmp_dir = setup_test_env();
    env::set_var("EDITOR", "-"); // Ensure editor is set to dash

    // Add a TODO first
    add_todo(Some("Show Test TODO".to_string()));

    // Get the TODO ID
    let connection = &mut establish_connection();
    use workingon::schema::todos::dsl::*;
    let results = todos
        .select(workingon::models::Todos::as_select())
        .filter(title.eq("Show Test TODO"))
        .load(connection)
        .expect("Error loading todos");

    let todo_id = encode_id(results[0].id.try_into().unwrap());

    // Show the TODO
    show_todo(todo_id);

    cleanup_test_env();
}

#[test]
#[serial]
fn test_list_todos() {
    let _tmp_dir = setup_test_env();
    env::set_var("EDITOR", "-"); // Ensure editor is set to dash

    // Add multiple TODOs
    add_todo(Some("First TODO".to_string()));
    add_todo(Some("Second TODO".to_string()));
    add_todo(Some("Third TODO".to_string()));

    // List TODOs
    list_todos(None, None);

    cleanup_test_env();
}

#[test]
#[serial]
fn test_delete_todo() {
    let _tmp_dir = setup_test_env();

    // Add a TODO first
    add_todo(Some("Delete Test TODO".to_string()));

    // Get the TODO ID
    let connection = &mut establish_connection();
    use workingon::schema::todos::dsl::*;
    let results = todos
        .select(workingon::models::Todos::as_select())
        .filter(title.eq("Delete Test TODO"))
        .load(connection)
        .expect("Error loading todos");

    let todo_id = encode_id(results[0].id.try_into().unwrap());

    // Delete the TODO
    delete_todo(todo_id);

    // Verify it was deleted
    let remaining = todos
        .select(workingon::models::Todos::as_select())
        .filter(title.eq("Delete Test TODO"))
        .load(connection)
        .expect("Error loading todos");
    assert_eq!(remaining.len(), 0);

    cleanup_test_env();
}

#[test]
#[serial]
fn test_edit_todo() {
    let _tmp_dir = setup_test_env();

    // Add a TODO first
    add_todo(Some("Edit Test TODO".to_string()));

    // Get the TODO ID
    let connection = &mut establish_connection();
    use workingon::schema::todos::dsl::*;
    let results = todos
        .select(workingon::models::Todos::as_select())
        .filter(title.eq("Edit Test TODO"))
        .load(connection)
        .expect("Error loading todos");

    let todo_id = encode_id(results[0].id.try_into().unwrap());

    // Edit the TODO
    edit_todo(todo_id);

    cleanup_test_env();
}

#[test]
#[should_panic(expected = "Couldn't decode id")]
fn test_decode_id_invalid_input() {
    decode_id("invalid_id");
}

#[test]
fn test_constants() {
    assert_eq!(
        workingon::COMMENT_DISCLAIMER,
        "# This is a comment, lines starting with a # will be ignored"
    );
    assert!(!workingon::constants::BIN.is_empty());
    assert!(!workingon::constants::DEFAULT_EDITOR.is_empty());
}
