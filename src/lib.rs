pub mod cli;
pub mod constants;
pub mod models;
pub mod schema;

use chrono::*;
use colored::Colorize;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dirs::data_dir;
use models::{NewTodo, Todos};
use sqids::Sqids;
use std::{
    io::{Read, Write},
    str::FromStr,
};

use self::constants::{BIN, DEFAULT_EDITOR};
use self::schema::todos;

// Constants only used in this file
pub const COMMENT_DISCLAIMER: &str = "# This is a comment, lines starting with a # will be ignored";
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn create_sqids_encoder_with_custom_alphabet() -> Sqids {
    Sqids::builder()
        .min_length(5)
        .alphabet("1234567890abcdefghijklmnopqrstuvwxyz".chars().collect())
        .build()
        .expect("Failed to create Sqids encoder with custom alphabet configuration")
}

pub fn encode_id(i: u64) -> String {
    create_sqids_encoder_with_custom_alphabet().encode(&vec![i]).expect("Problem encoding id")
}

pub fn decode_id(s: &str) -> i32 {
    // TODO can I make this nicer?
    (*create_sqids_encoder_with_custom_alphabet().decode(s).first().expect("Couldn't decode id"))
        .try_into()
        .unwrap()
}

// Path-related functions
pub fn get_project_data_folder() -> std::path::PathBuf {
    let env_var_name = format!("{}_data_dir", BIN).to_uppercase();
    match std::env::var(env_var_name) {
        Ok(dd) => std::path::PathBuf::from_str(&dd).expect("Env var data dir is not a valid path"),
        Err(_) => {
            let mut data_folder = data_dir().expect("Couldn't get data dir");
            data_folder.push(BIN);
            if !data_folder.exists() {
                std::fs::create_dir_all(data_folder.as_path())
                    .expect("Wasn't able to create the folder {data_folder}");
            }
            data_folder
        }
    }
}

pub fn get_db_file() -> std::path::PathBuf {
    let mut db_file = get_project_data_folder();
    db_file.push("todos.sqlite3");
    db_file
}

pub fn get_todoeditmsg_file() -> std::path::PathBuf {
    let mut todo_file = get_project_data_folder();
    // TODO: we should clean this up if it's left behind at startup
    todo_file.push("TODO_EDITMSG");
    todo_file
}

pub fn get_editor() -> String {
    let editor = std::env::var("EDITOR");
    match editor {
        Ok(editor) => editor,
        Err(_) => DEFAULT_EDITOR.to_string(),
    }
}

// Database operations
pub fn establish_connection() -> SqliteConnection {
    let database_url = get_db_file().display().to_string();
    let mut conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    //TODO: a match here could perform log a message for successful migrations
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Migrations couldn't be run");
    conn
}

pub fn create_temp_todo_file_open_and_then_read_remove_process(
    fp: &std::path::Path,
    title: String,
    notes: String,
) -> (String, String) {
    let body = format!(
            "{}
{}
{}

# The first non-comment line will assumed to be the title and every other line will be saved as notes
",
        title,
        COMMENT_DISCLAIMER,
        notes,
    );
    let mut final_title = title;
    let mut full_notes = notes;
    let editor = get_editor();
    if editor != "-" {
        let mut file = std::fs::File::create(fp)
            .unwrap_or_else(|_| panic!("File {} couldn't be created", fp.display()));
        file.write_all(body.as_bytes())
            .unwrap_or_else(|_| panic!("the body couldn't be written to {}", fp.display()));
        std::process::Command::new(get_editor())
            .arg(fp)
            .status()
            .unwrap_or_else(|_| panic!("opening editor for {} failed", fp.display()));
        let mut buf = String::new();
        std::fs::File::open(fp)
            .unwrap_or_else(|_| panic!("opening {} for reading after editing failed", fp.display()))
            .read_to_string(&mut buf)
            .unwrap_or_else(|_| panic!("reading {} after editing failed", fp.display()));
        std::fs::remove_file(fp)
            .unwrap_or_else(|_| panic!("{} couldn't be removed once it was read", fp.display()));
        // TODO: maybe rename notes to body?
        let mut not_comments = buf.lines().filter(|e| !e.trim_start().starts_with("#"));
        final_title = not_comments
            .next()
            .expect("Couldn't find title of new TODO")
            .to_string();
        let notes: Vec<&str> = not_comments.collect();
        full_notes = notes.join("\n");
        full_notes = full_notes
            .trim_start_matches('\n')
            .trim_end_matches('\n')
            .to_string();
        // TODO: what if file had nothing in it? What if I removed title, maybe cancel?
    }

    (final_title.to_string(), full_notes)
}

// High-level database operations
pub fn add_todo(todo: &NewTodo) -> Todos {
    let connection = &mut establish_connection();
    diesel::insert_into(todos::table)
        .values(todo)
        .returning(Todos::as_returning())
        .get_result(connection)
        .expect("Error saving new TODO")
}

pub fn get_todo(get_id: &String) -> Todos {
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    let decoded_id = decode_id(get_id);
    todos.select(Todos::as_select())
        .filter(id.eq(decoded_id))
        .first(connection)
        .unwrap_or_else(|_| panic!("Single TODO couldn't be found with id {}", get_id))
}

pub fn get_todos() -> Vec<Todos> {
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    todos.select(Todos::as_select())
        .load(connection)
        .expect("Was unable to get all TODOs")
}

pub fn complete_todo(show_id: &String, ts: Option<DateTime<Utc>>) {
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    let decoded_id = decode_id(show_id);
    let completion_ts = ts.unwrap_or_else(|| Utc::now());
    diesel::update(todos.find(decoded_id))
        .set(completed.eq(completion_ts))
        .execute(connection)
        .unwrap_or_else(|_| panic!("TODO: {} couldn't be completed", show_id));
}

pub fn set_todo_title(update_id: &String, new_title: &String) {
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    let decoded_id = decode_id(update_id);
    diesel::update(todos.find(decoded_id))
        .set(title.eq(new_title))
        .execute(connection)
        .unwrap_or_else(|_| panic!("title of TODO: {} couldn't be updated", update_id));
}

pub fn set_todo_notes(update_id: &String, new_notes: &String) {
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    let decoded_id = decode_id(update_id);
    diesel::update(todos.find(decoded_id))
        .set(notes.eq(new_notes))
        .execute(connection)
        .unwrap_or_else( |_| panic!("notes of TODO: {} couldn't be updated", update_id));
}

pub fn reopen_todo(show_id: &String) {
    use self::schema::todos::dsl::*;
    use chrono::DateTime;
    let connection = &mut establish_connection();
    let decoded_id = decode_id(show_id);
    diesel::update(todos.find(decoded_id))
        .set(completed.eq(None::<DateTime<Utc>>))
        .execute(connection)
        .expect("TODO couldn't be reopened");
    println!("{} reopened, if this was a mistake complete with `{} complete {}`", show_id.yellow(), BIN, show_id)
}

pub fn delete_todo(delete_id: &String) {
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    let decoded_id = decode_id(delete_id);
    diesel::delete(todos.filter(id.eq(decoded_id)))
        .execute(connection)
        .expect("Error loading posts");
}
