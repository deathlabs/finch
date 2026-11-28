use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::error::Error;
use serde::Serialize;

use crate::model::MappingsFile;
use crate::ops::{
    self, CreateComponent, DeleteComponent, DeleteControl, DeleteRule, GetComponent, GetControl,
    GetRule, MapControl, MapRule,
};
use crate::{mcp, schema, store, tui};

#[derive(Parser)]
#[command(about = "Shape what your scanners can see.", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Start an interactive interface.")]
    Start {
        #[command(subcommand)]
        interface: StartInterface,
    },
    #[command(about = "Get information about mappings files.")]
    Get {
        #[command(subcommand)]
        entry: GetEntry,
    },
    #[command(about = "Add components, controls, and rules to a mappings file.")]
    Add {
        #[arg(long, value_name = "PATH", default_value = "mappings.yaml")]
        mappings_file: PathBuf,
        #[command(subcommand)]
        entry: AddEntry,
    },
    #[command(about = "Delete components, controls, and rules from a mappings file.")]
    Delete {
        #[arg(long, value_name = "PATH", default_value = "mappings.yaml")]
        mappings_file: PathBuf,
        #[command(subcommand)]
        entry: DeleteEntry,
    },
}

#[derive(Subcommand)]
enum StartInterface {
    /// Start the TUI
    Tui {
        #[arg(short, long, value_name = "PATH")]
        ssp: Option<PathBuf>,
    },
    /// Start the MCP server over HTTP
    Mcp {
        #[arg(long, default_value_t = 8765)]
        port: u16,
        #[arg(long, value_name = "PATH", default_value = "mappings.yaml")]
        mappings_file: PathBuf,
    },
}

#[derive(Subcommand)]
enum GetEntry {
    #[command(about = "Print the schema for mappings files in JSON.")]
    Schema,
    #[command(about = "Print a component and its mappings.")]
    Component {
        #[command(flatten)]
        input: GetComponent,
        #[command(flatten)]
        options: GetOptions,
    },
    #[command(about = "Print an implemented control ID.")]
    Control {
        #[command(flatten)]
        input: GetControl,
        #[command(flatten)]
        options: GetOptions,
    },
    #[command(about = "Print a rule mapping.")]
    Rule {
        #[command(flatten)]
        input: GetRule,
        #[command(flatten)]
        options: GetOptions,
    },
}

#[derive(Args)]
struct GetOptions {
    #[arg(long, value_name = "PATH", default_value = "mappings.yaml")]
    mappings_file: PathBuf,
    #[arg(short = 'o', long, value_enum, default_value = "json")]
    output_format: OutputFormat,
}

#[derive(Clone, Copy, ValueEnum)]
enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Subcommand)]
enum AddEntry {
    /// Add a named component.
    Component(CreateComponent),
    /// Add a control to an existing component
    Control(MapControl),
    /// Add a rule to an existing component
    Rule(MapRule),
}

#[derive(Subcommand)]
enum DeleteEntry {
    /// Delete a component
    Component(DeleteComponent),
    /// Delete a control from a component
    Control(DeleteControl),
    /// Delete a rule mapping from a component
    Rule(DeleteRule),
}

pub fn run() -> ExitCode {
    let cli: Cli = Cli::parse();
    let result: Result<(), Error> = match cli.command {
        Command::Add {
            mappings_file,
            entry,
        } => store::update(&mappings_file, |document| {
            return match entry {
                AddEntry::Component(input) => {
                    ops::create_component(&mut document.components, &input)
                }
                AddEntry::Control(input) => ops::map_control(&mut document.components, &input),
                AddEntry::Rule(input) => ops::map_rule(&mut document.components, &input),
            };
        }),
        Command::Delete {
            mappings_file,
            entry,
        } => store::update(&mappings_file, |document| {
            return match entry {
                DeleteEntry::Component(input) => {
                    ops::delete_component(&mut document.components, &input)
                }
                DeleteEntry::Control(input) => {
                    ops::delete_control(&mut document.components, &input)
                }
                DeleteEntry::Rule(input) => ops::delete_rule(&mut document.components, &input),
            };
        }),
        Command::Start { interface } => {
            match interface {
                StartInterface::Tui { ssp } => {
                    if let Err(error) = tui::run(ssp) {
                        eprintln!("Failed to run the TUI: {error}");
                        return ExitCode::FAILURE;
                    }
                }
                StartInterface::Mcp {
                    port,
                    mappings_file,
                } => {
                    if mcp::run(port, &mappings_file).is_err() {
                        return ExitCode::FAILURE;
                    }
                }
            }
            return ExitCode::SUCCESS;
        }
        Command::Get {
            entry: GetEntry::Component { input, options },
        } => print_entry(&options, |document| {
            return ops::get_component(&document.components, &input);
        }),
        Command::Get {
            entry: GetEntry::Control { input, options },
        } => print_entry(&options, |document| {
            return ops::get_control(&document.components, &input);
        }),
        Command::Get {
            entry: GetEntry::Rule { input, options },
        } => print_entry(&options, |document| {
            return ops::get_rule(&document.components, &input);
        }),
        Command::Get {
            entry: GetEntry::Schema,
        } => {
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
            return ExitCode::SUCCESS;
        }
    };
    if let Err(error) = result {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }
    return ExitCode::SUCCESS;
}

fn print_entry<T: Serialize + ?Sized>(
    options: &GetOptions,
    find: impl FnOnce(&MappingsFile) -> Result<&T, Error>,
) -> Result<(), Error> {
    let document: MappingsFile = store::load(&options.mappings_file)?;
    let entry: &T = find(&document)?;
    let mut output: io::StdoutLock<'_> = io::stdout().lock();
    match options.output_format {
        OutputFormat::Json => {
            serde_json::to_writer_pretty(&mut output, entry).map_err(io::Error::from)?;
            writeln!(output)?;
        }
        OutputFormat::Yaml => serde_yaml_ng::to_writer(&mut output, entry)?,
    }
    output.flush()?;
    return Ok(());
}
