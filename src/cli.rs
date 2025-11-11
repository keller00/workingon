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
    /// Locate database file
    #[clap(hide = true)]
    LocateDb,
    /// Add a new TODO
    Add {
        /// title of the new TODO
        #[clap()]
        title: Option<String>,
    },
    /// List current TODOs, flag priority: all > completed > open (default).
    #[clap(visible_alias = "ls")]
    List {
        /// show only completed TODOs
        #[arg(long, action = clap::builder::ArgAction::SetTrue)]
        completed: bool,
        /// show only open TODOs (this is the default behavior)
        #[arg(long, action = clap::builder::ArgAction::SetTrue)]
        open: bool,
        /// show both completed and open TODOs, overwrites other flags
        #[arg(long, action = clap::builder::ArgAction::SetTrue)]
        all: bool,
    },
    #[clap(visible_alias = "rm")]
    /// Remove a TODO
    Delete {
        #[clap()]
        id: String,
    },
    #[clap()]
    /// Show information about a TODO
    Show {
        #[clap()]
        id: String,
    },
    #[clap()]
    /// Edit a TODO
    Edit {
        #[clap()]
        id: String,
    },
    #[clap()]
    /// Complete a TODO
    Complete {
        #[clap()]
        id: String,
    },
    #[clap()]
    /// Reopen a done TODO
    Reopen {
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
        Some(Commands::List { all, completed, open: _ }) => {
            // Priority: --all > --completed > default (--open)
            if *all {
                // Show all TODOs
                crate::list_todos(Some(false));
            } else if *completed {
                // Show only completed TODOs
                crate::list_todos(Some(true));
            } else {
                // Default: show open (uncompleted) TODOs
                crate::list_todos(None);
            }
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
        Some(Commands::Reopen { id }) => {
            crate::reopen_todo(id.to_string());
        }
        None => {}
    }
}
