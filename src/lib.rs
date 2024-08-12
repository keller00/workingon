pub mod models;
pub mod schema;

use self::schema::todos;

use chrono::Utc;
use clap::{Parser, Subcommand};
use dirs::data_dir;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use models::{NewTodo, Todos};

const BIN: &str = env!("CARGO_PKG_NAME");
const BIN_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    disable_version_flag = true,
    disable_help_subcommand = true,
    author,
    version=get_version_str(),
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
    AddTodo {
        /// title of the new TODO
        #[clap()]
        title: String,
    },
    /// list current TODOs
    ListTodos,
}

fn get_version_str() -> String {
    return format!("version {}", BIN_VERSION);
}

fn print_version() {
    println!("{} {}", BIN, get_version_str());
}

fn get_db_file() -> std::path::PathBuf{
    let mut db_file = data_dir().expect("Couldn't get data dir");
    db_file.push("events.sqlite3");
    return db_file
}

fn get_todo_file() -> std::path::PathBuf{
    let mut todo_file = data_dir().expect("Couldn't get data dir");
    todo_file.push("TODO");
    return todo_file
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
    conn.run_pending_migrations(MIGRATIONS).unwrap();
    return conn;
}

pub fn add_todo(title: String) {
    let connection = &mut establish_connection();
    let new_todo = NewTodo{title: &title, notes: "", created_on: Utc::now()};
    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todos::as_returning())
        .get_result(connection)
        .expect("Error saving new TODO");
}

pub fn list_todos(){
    use self::schema::todos::dsl::*;
    let connection = &mut establish_connection();
    let results = todos
        .limit(5)
        .select(Todos::as_select())
        .load(connection)
        .expect("Error loading posts");
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.notes);
    }
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
        Some(Commands::AddTodo { title }) => {
            add_todo(title.to_string());
        }
        Some(Commands::ListTodos) => {
            list_todos();
        }
        None => {}
    }
}
