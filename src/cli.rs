use crate::constants::{BIN, BIN_VERSION};
use clap::{Parser, Subcommand};

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
    List {
        #[arg(long, action = clap::builder::ArgAction::SetTrue)]
        show_completed: bool,
    },
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
    #[clap()]
    Complete {
        #[clap()]
        id: String,
    },
}

fn get_version_str() -> String {
    format!("version {}", BIN_VERSION)
}

fn print_version() {
    println!("{} {}", BIN, get_version_str());
}

fn print_db_file() {
    println!("{}", crate::get_db_file().display())
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
            crate::add_todo(title.clone());
        }
        Some(Commands::List { show_completed }) => {
            crate::list_todos(if *show_completed { Some(true) } else { None });
        }
        Some(Commands::Delete { id }) => {
            crate::delete_todo(id.to_string());
        }
        Some(Commands::Show { id }) => {
            crate::show_todo(id.to_string());
        }
        Some(Commands::Edit { id }) => {
            crate::edit_todo(id.to_string());
        }
        Some(Commands::Complete { id }) => {
            crate::complete_todo(id.to_string());
        }
        None => {}
    }
}
