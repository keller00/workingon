use crate::constants::{BIN, BIN_VERSION};
use crate::models::NewTodo;

use colored::Colorize;
use chrono::{DateTime, Utc, Local};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    disable_version_flag = true,
    disable_help_subcommand = true,
    author,
    version = get_version_str(),
    about,
    long_about = None,
    arg_required_else_help = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
        /// close the TODO right after creation
        #[clap(short, long, action)]
        complete: bool,
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
    #[clap(visible_alias = "done")]
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

// TODO: make this private?
pub fn run_cli() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Version => {
            println!("{} {}", BIN, get_version_str());
        }
        Commands::LocateDb => {
            println!("{}", crate::get_db_file().display());
        }
        Commands::Add { title, complete } => {
            add_todo(title, complete);
        }
        Commands::List { all, completed, open: _ } => {
            // Priority: --all > --completed > default (--open)
            if all {
                // Show all TODOs
                list_todos(Some(false));
            } else if completed {
                // Show only completed TODOs
                list_todos(Some(true));
            } else {
                // Default: show open (uncompleted) TODOs
                list_todos(None);
            }
        }
        Commands::Delete { id } => {
            delete_todo(&id.to_string());
        }
        Commands::Show { id } => {
            show_todo(&id.to_string());
        }
        Commands::Edit { id } => {
            edit_todo(id.to_string());
        }
        Commands::Complete { id } => {
            complete_todo(&id.to_string());
        }
        Commands::Reopen { id } => {
            reopen_todo(&id.to_string());
        }
    }
}

fn format_datetime(ts: DateTime<Utc>, precise: bool) -> String {
    let local_tz = ts.with_timezone(&Local);
    if !precise {
        let ht = chrono_humanize::HumanTime::from(ts);
        return format!("{}", ht);

    }
    format!("{}", local_tz.format("%d/%m/%Y %H:%M"))
}

fn format_datetime_or_else(ts: Option<DateTime<Utc>>, else_item: String, precise: bool) -> String {
    match ts {
        Some(ts) => {
            format_datetime(ts, precise)
        }
        None => {
            else_item
        }
    }
}

fn show_todo(id: &String) {
    let found_todo = crate::get_todo(id);
    let created_str: String = format_datetime(found_todo.created, false);
    let completed_str: String = format_datetime_or_else(found_todo.completed, "not yet".to_string(), false);
    println!("{}\n{}\nIt was created: {}\nIt was completed: {}",
        found_todo.title,
        found_todo.notes,
        created_str,
        completed_str,
    );
}

pub fn edit_todo(id: String) {
    let found_todo = crate::get_todo(&id);
    let p_buff = crate::get_todoeditmsg_file();
    let fp = p_buff.as_path();
    let (t, n) = crate::create_temp_todo_file_open_and_then_read_remove_process(
        fp,
        found_todo.title.clone(),
        found_todo.notes.clone(),
    );
    crate::set_todo_title(&id, &t);
    crate::set_todo_notes(&id, &n);
    println!("{} updated", id.yellow());
}

fn complete_todo(id: &String) {
    crate::complete_todo(id, None);
    println!("{} completed, if this was a mistake reopen with `{} reopen {}`", id.yellow(), BIN, id)
}

fn reopen_todo(id: &String) {
    crate::reopen_todo(id);
    println!("{} reopened, if this was a mistake complete with `{} complete {}`", id.yellow(), BIN, id)
}

pub fn delete_todo(id: &String) {
    crate::delete_todo(id);
    println!("{} deleted", id.yellow());
}

pub fn add_todo(title: Option<String>, complete_after_creation: bool) {
    // TODO: There should be a way to supply body easily just like in `git commit -m ""`, but
    //  don't forget multiline messages with multiple -m's
    let title_str = match title {
        Some(t) => t,
        None => "<title>".to_string(),
    };
    let p_buff = crate::get_todoeditmsg_file();
    let fp = p_buff.as_path();
    let (title, notes) =
        crate::create_temp_todo_file_open_and_then_read_remove_process(fp, title_str, String::new());
    let new_todo = NewTodo {
        title: title.as_str(),
        notes: notes.as_str(),
        created: Utc::now(),
    };
    let created_todo = crate::add_todo(&new_todo);
    if complete_after_creation {
        crate::complete_todo(
            &crate::encode_id(created_todo.id.try_into().unwrap()),
            Some(created_todo.created),
        );

    }
    println!(
        "{} created{}",
        crate::encode_id(created_todo.id.try_into().unwrap()).yellow(),
        if complete_after_creation {
            " and was subsequently completed"
        } else {""}
    );
}

pub fn list_todos(show_completed: Option<bool>) {
    let mut results = crate::get_todos();
    
    // show_completed parameter:
    // - None: show open (uncompleted) TODOs (default behavior)
    // - Some(true): show only completed TODOs
    // - Some(false): show all TODOs (both completed and open)
    match show_completed {
        Some(true) => {
            // Show only completed TODOs
            results.retain(|todo| todo.completed.is_some());
        }
        Some(false) => {
            // Show all TODOs (no filter)
        }
        None => {
            // Default: show only open (uncompleted) TODOs
            results.retain(|todo| todo.completed.is_none());
        }
    }
    
    // Sort by id descending (same as the original database query)
    results.sort_by(|a, b| b.id.cmp(&a.id));
    
    if results.is_empty() {
        println!("There's nothing to do currently :) Add a new one with `{} add`", BIN);
    } else {
        let mut table = comfy_table::Table::new();
        table.load_preset(comfy_table::presets::NOTHING);
        table.set_header(vec!["id", "created", "title"]);
        for post in results {
            table.add_row(vec![
                comfy_table::Cell::new(
                    // With custom_styling comfy_table flag we can keep using colorize colors, but
                    // slow down comfy table by 30-50%. I think this is acceptable for now, but
                    // could later switch to using comfy_table's built-in coloring.
                    crate::encode_id(post.id.try_into().expect("Failed to cast post id in list")).yellow().to_string(),
                ),
                comfy_table::Cell::new(
                    format_datetime(post.created, false),
                ),
                comfy_table::Cell::new(post.title),
            ]);
        }
        table.column_mut(2).unwrap().set_constraint(
            comfy_table::ColumnConstraint::UpperBoundary(comfy_table::Width::Percentage(60))
        );
        println!("{table}")
    }
}
