pub mod models;
pub mod schema;

use std::io::{Read, Write};

use self::schema::todos;

use chrono::Utc;
use clap::{Parser, Subcommand};
use colored::Colorize;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dirs::data_dir;
use models::{NewTodo, Todos};
use sqids::Sqids;

const BIN: &str = env!("CARGO_PKG_NAME");
const BIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_EDITOR: &str = "vi";

const COMMENT_DISCLAIMER: &str = "# This is a comment, lines starting with a # will be ignored";

#[derive(Parser)]
#[command(
    disable_version_flag = true,
    disable_help_subcommand = true,
    author,
    version = get_version_str(),
    about,
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(
        short = 'v',
        long,
        help = "Print version",
        action = clap::builder::ArgAction::Version,
    )]
    version: (),
}

#[derive(Subcommand)]
enum Commands {
    /// Print version
    Version,
    /// locate database file
    #[clap(hide = true)]
    LocateDb,
    /// add a TODO
    Add {
        /// title of the new TODO
        #[clap()]
        title: Option<String>,
    },
    /// list current TODOs
    #[clap(visible_alias = "ls")]
    List,
    #[clap(visible_alias = "rm")]
    Delete {
        #[clap()]
        id: String,
    },
    #[clap()]
    Show {
        #[clap()]
        id: String,
    },
    #[clap()]
    Edit {
        #[clap()]
        id: String,
    },
}

fn get_squids() -> Sqids {
    Sqids::builder()
        .min_length(5)
        .alphabet("1234567890abcdefghijklmnopqrstuvwxyz".chars().collect())
        .build()
        .expect("Couldn't get squids")
}

fn encode_id(i: u64) -> String {
    get_squids().encode(&vec![i]).expect("Problem encoding id")
}

fn decode_id(s: &str) -> i32 {
    // TODO can I make this nicer?
    (*get_squids()
        .decode(s)
        .first()
        .expect("Couldn't decode id"))
    .try_into()
    .unwrap()
}

fn get_version_str() -> String {
    format!("version {}", BIN_VERSION)
}

fn print_version() {
    println!("{} {}", BIN, get_version_str());
}

// TODO: make this return a Path
fn get_project_data_folder() -> std::path::PathBuf {
    let mut data_folder = data_dir().expect("Couldn't get data dir");
    data_folder.push(BIN);
    if !data_folder.exists() {
        std::fs::create_dir_all(data_folder.as_path())
            .expect("Wasn't able to create the folder {data_folder}");
    }
    data_folder
}

fn get_db_file() -> std::path::PathBuf {
    let mut db_file = get_project_data_folder();
    db_file.push("todos.sqlite3");
    db_file
}

fn get_todoeditmsg_file() -> std::path::PathBuf {
    let mut todo_file = get_project_data_folder();
    // TODO: we should clean this up if it's left behind at startup
    todo_file.push("TODO_EDITMSG");

    todo_file
}

fn get_editor() -> String {
    let editor = std::env::var("EDITOR");
    match editor {
        Ok(editor) => editor,
        Err(_) => DEFAULT_EDITOR.to_string(),
    }
}

fn print_db_file() {
    println!("{}", get_db_file().display())
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = get_db_file().display().to_string();
    let mut conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    //TODO: a match here could perform log a message for successful migrations
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Migrations couldn't be run");
    conn
}

pub fn create_temp_todo_file_open_and_then_read_remove_process(
    fp: &std::path::Path,
    body: String,
) -> (String, String) {
    let mut file = std::fs::File::create(fp)
        .expect(format!("File {} couldn't be created", fp.display()).as_str());
    file.write_all(body.as_bytes())
        .expect(format!("the body couldn't be written to {}", fp.display()).as_str());
    std::process::Command::new(get_editor())
        .arg(fp)
        .status()
        .expect(format!("opening editor for {} failed", fp.display()).as_str());
    let mut buf = String::new();
    std::fs::File::open(fp)
        .expect(format!("opening {} for reading after editing failed", fp.display()).as_str())
        .read_to_string(&mut buf)
        .expect(format!("reading {} after editing failed", fp.display()).as_str());
    std::fs::remove_file(fp)
        .expect(format!("{} couldn't be removed once it was read", fp.display()).as_str());
    // TODO: maybe rename notes to body?
    let mut not_comments = buf.lines().filter(|e| !e.trim_start().starts_with("#"));
    let final_title = not_comments
        .next()
        .expect("Couldn't find title of new TODO");
    let notes: Vec<&str> = not_comments.collect();
    let mut full_notes = notes.join("\n");
    full_notes = full_notes
        .trim_start_matches('\n')
        .trim_end_matches('\n')
        .to_string();
    // TODO: what if file had nothing in it? What if I removed title, maybe cancel?

    (final_title.to_string(), full_notes)
}

pub fn add_todo(title: Option<String>) {
    // TODO: There should be a way to supply body easily just like in `git commit -m ""`, but
    //  don't forget multiline messages with multiple -m's
    let title_str = match title {
        Some(t) => t,
        None => "<title>".to_string(),
    };
    let p_buff = get_todoeditmsg_file();
    let fp = p_buff.as_path();
    let body = format!(
            "{}

{}

# The first non-comment line will assumed to be the title and every other line will be saved as notes
",
        title_str,
        COMMENT_DISCLAIMER,
    );
    let (title, notes) = create_temp_todo_file_open_and_then_read_remove_process(fp, body);
    let connection = &mut establish_connection();
    let new_todo = NewTodo {
        title: title.as_str(),
        notes: notes.as_str(),
        created_on: Utc::now(),
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
    println!("{}\n{}", found_todos[0].title, found_todos[0].notes);
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
    let body = format!(
            "{}
{}
{}

# The first non-comment line will assumed to be the title and every other line will be saved as notes
",
        found_todos[0].title,
        COMMENT_DISCLAIMER,
        found_todos[0].notes,
    );
    let (t, n) = create_temp_todo_file_open_and_then_read_remove_process(fp, body);
    // TODO: do the rest of this
    diesel::update(&found_todos[0])
        .set((title.eq(t), notes.eq(n)))
        .execute(connection)
        .expect("Was unable to update TODO");
    println!("TODO updated successfully")
}

pub fn list_todos() {
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    let results = todos
        .select(Todos::as_select())
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

// TODO: make this private?
pub fn run_cli() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Version) => {
            print_version();
        }
        Some(Commands::LocateDb) => {
            print_db_file();
        }
        Some(Commands::Add { title }) => {
            // TODO: maybe don't clone here
            add_todo(title.clone());
        }
        Some(Commands::List {}) => {
            list_todos();
        }
        Some(Commands::Delete { id }) => {
            delete_todo(id.to_string());
        }
        Some(Commands::Show { id }) => {
            show_todo(id.to_string());
        }
        Some(Commands::Edit { id }) => {
            edit_todo(id.to_string());
        }
        None => {}
    }
}
