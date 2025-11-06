pub mod cli;
pub mod constants;
pub mod models;
pub mod schema;

use chrono::Utc;
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

fn get_squids() -> Sqids {
    Sqids::builder()
        .min_length(5)
        .alphabet("1234567890abcdefghijklmnopqrstuvwxyz".chars().collect())
        .build()
        .expect("Couldn't get squids")
}

pub fn encode_id(i: u64) -> String {
    get_squids().encode(&vec![i]).expect("Problem encoding id")
}

pub fn decode_id(s: &str) -> i32 {
    // TODO can I make this nicer?
    (*get_squids().decode(s).first().expect("Couldn't decode id"))
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
pub fn add_todo(title: Option<String>) {
    // TODO: There should be a way to supply body easily just like in `git commit -m ""`, but
    //  don't forget multiline messages with multiple -m's
    let title_str = match title {
        Some(t) => t,
        None => "<title>".to_string(),
    };
    let p_buff = get_todoeditmsg_file();
    let fp = p_buff.as_path();
    let (title, notes) =
        create_temp_todo_file_open_and_then_read_remove_process(fp, title_str, String::new());
    let connection = &mut establish_connection();
    let new_todo = NewTodo {
        title: title.as_str(),
        notes: notes.as_str(),
        created: Utc::now(),
    };
    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todos::as_returning())
        .get_result(connection)
        .expect("Error saving new TODO");
    println!("TODO added successfully");
}

pub fn show_todo(show_id: String) {
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    let decoded_id = decode_id(&show_id);
    let found_todos = todos
        .select(Todos::as_select())
        .filter(id.eq(decoded_id))
        .load(connection)
        .expect("TODOs couldn't be retrieved");
    assert!(
        found_todos.len() == 1,
        "TODO to show couldn't be found {}",
        found_todos.len()
    );
    let completed_str = found_todos[0].completed.map(|c| c.to_string()).unwrap_or("not yet".to_string());
    println!("{}\nIt was completed on: {}\n{}",
        found_todos[0].title,
        completed_str,
        found_todos[0].notes,
    );
}

pub fn complete_todo(show_id: String) {
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    let decoded_id = decode_id(&show_id);
    diesel::update(todos.find(decoded_id))
        .set(completed.eq(Utc::now()))
        .execute(connection)
        .expect("TODO couldn't be completed");
    println!("TODO was completed")
}

pub fn edit_todo(show_id: String) {
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    let decoded_id = decode_id(&show_id);
    let found_todos = todos
        .select(Todos::as_select())
        .filter(id.eq(decoded_id))
        .load(connection)
        .expect("TODOs couldn't be retrieved");
    assert!(
        found_todos.len() == 1,
        "TODO to show couldn't be found {}",
        found_todos.len()
    );
    let p_buff = get_todoeditmsg_file();
    let fp = p_buff.as_path();
    let (t, n) = create_temp_todo_file_open_and_then_read_remove_process(
        fp,
        found_todos[0].title.clone(),
        found_todos[0].notes.clone(),
    );
    // TODO: do the rest of this
    diesel::update(&found_todos[0])
        .set((title.eq(t), notes.eq(n)))
        .execute(connection)
        .expect("Was unable to update TODO");
    println!("TODO updated successfully")
}

pub fn list_todos(show_completed: Option<bool>) {
    let s_completed = show_completed.unwrap_or(false);
    use self::schema::todos::dsl::*;
    use diesel::sqlite::Sqlite;
    let connection = &mut establish_connection();
    let mut query = todos.select(Todos::as_select()).into_boxed::<Sqlite>();
    if !s_completed {
        query = query.filter(completed.is_null());
    }
    let results = query
        .order_by(id.desc())
        .limit(5)
        .load(connection)
        .expect("Error loading posts");
    for post in results {
        println!(
            "{} {}",
            encode_id(post.id.try_into().unwrap()).yellow(),
            post.title
        );
    }
}

pub fn delete_todo(delete_id: String) {
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    let decoded_id = decode_id(&delete_id);
    diesel::delete(todos.filter(id.eq(decoded_id)))
        .execute(connection)
        .expect("Error loading posts");
    println!("Post with id {} was deleted", delete_id);
}
