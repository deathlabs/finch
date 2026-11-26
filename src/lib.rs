mod tui;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(about = "Shape what your scanners can see.", version)]
struct Cli {
    #[arg(long, value_enum, default_value = "cli", help = "Select the mode")]
    mode: Mode,

    #[arg(
        short,
        long,
        value_name = "PATH",
        help = "Open an OSCAL System Security Plan in TUI mode"
    )]
    ssp: Option<PathBuf>,
}

#[derive(Clone, ValueEnum)]
enum Mode {
    Cli,
    Tui,
    Mcp,
}

pub fn run() -> ExitCode {
    let cli: Cli = Cli::parse();
    match cli.mode {
        Mode::Cli => println!("CLI mode"),
        Mode::Tui => {
            if let Err(error) = tui::run(cli.ssp) {
                eprintln!("Failed to run in TUI mode: {error}");
                return ExitCode::FAILURE;
            }
        }
        Mode::Mcp => println!("MCP mode"),
    }
    return ExitCode::SUCCESS;
}
