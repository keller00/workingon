use clap::{Parser, Subcommand};

const BIN: &str = env!("CARGO_PKG_NAME");
const BIN_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Make the output more verbose
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print version
    Version,
}

fn print_version() {
    println!("{BIN} version {BIN_VERSION}");
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Version) => {
            print_version();
        }
        None => {}
    }
}
