use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::ops::{self, CreateComponent, MapControl, MapRule};
use crate::{schema, store, tui};

#[derive(Parser)]
#[command(about = "Shape what your scanners can see.", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Open the terminal interface.
    Tui {
        #[arg(short, long, value_name = "PATH")]
        ssp: Option<PathBuf>,
    },
    /// Run in MCP mode.
    Mcp,
    /// Print the mappings file JSON Schema.
    Schema,
    /// Add an entry to a mappings file.
    Create {
        #[arg(long, value_name = "PATH", default_value = "mappings.yaml")]
        mappings_file: PathBuf,
        #[command(subcommand)]
        entry: CreateEntry,
    },
}

#[derive(Subcommand)]
enum CreateEntry {
    /// Add a named component.
    Component(CreateComponent),
    /// Associate a control with an existing component.
    Control(MapControl),
    /// Associate a scanner rule with a component's control.
    Rule(MapRule),
}

pub fn run() -> ExitCode {
    let cli: Cli = Cli::parse();
    match cli.command {
        Command::Create {
            mappings_file,
            entry,
        } => {
            let result = store::update(&mappings_file, |document| {
                return match entry {
                    CreateEntry::Component(input) => {
                        ops::create_component(&mut document.components, &input)
                    }
                    CreateEntry::Control(input) => {
                        ops::map_control(&mut document.components, &input)
                    }
                    CreateEntry::Rule(input) => ops::map_rule(&mut document.components, &input),
                };
            });
            if let Err(error) = result {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        }
        Command::Tui { ssp } => {
            if let Err(error) = tui::run(ssp) {
                eprintln!("Failed to run the TUI: {error}");
                return ExitCode::FAILURE;
            }
        }
        Command::Schema => {
            let result: io::Result<()> = (|| {
                let mut output: io::StdoutLock<'_> = io::stdout().lock();
                serde_json::to_writer_pretty(&mut output, &schema::mappings())?;
                writeln!(output)?;
                return Ok(());
            })();
            if let Err(error) = result {
                eprintln!("Failed to write the mappings schema: {error}");
                return ExitCode::FAILURE;
            }
        }
        Command::Mcp => println!("MCP mode"),
    }
    return ExitCode::SUCCESS;
}
