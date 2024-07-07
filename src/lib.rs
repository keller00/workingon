use std::str::FromStr;

use clap::{Parser, Subcommand};

const BIN: &str = env!("CARGO_PKG_NAME");
const BIN_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print version
    Version,
}

fn get_version_str() -> String {
    return format!("{} version {}", BIN, BIN_VERSION);
}

fn print_version() {
    println!("{}", get_version_str());
}

// TODO: make this private?
pub fn run_cli() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Version) => {
            print_version();
        }
        None => {}
    }
}
