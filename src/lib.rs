use clap::{Parser, Subcommand};

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
}

fn get_version_str() -> String {
    return format!("version {}", BIN_VERSION);
}

fn print_version() {
    println!("{} {}", BIN, get_version_str());
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
