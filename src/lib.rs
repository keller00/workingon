use clap::{Parser, Subcommand};
use dirs::data_dir;

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
    // TODO: hide this
    /// locate database file
    LocateDb,
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

fn print_db_file() {
    println!("{}", get_db_file().display())
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
        None => {}
    }
}
