mod mappings;
mod tui;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand, ValueEnum};

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

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Add an entry to a mappings file.
    Create {
        #[command(subcommand)]
        entry: CreateEntry,
    },
}

#[derive(Subcommand)]
enum CreateEntry {
    /// Add a named component to an existing mappings file.
    Component {
        #[arg(long, value_name = "PATH")]
        mappings_file: PathBuf,
        #[arg(long, value_name = "NAME")]
        component_name: String,
    },
    /// Associate a control or scanner rule with an existing component.
    Mapping {
        #[arg(long, value_name = "PATH")]
        mappings_file: PathBuf,
        #[arg(long, value_name = "NAME")]
        component_name: String,
        #[arg(long, value_name = "ID")]
        control: String,
        #[command(flatten)]
        rule_mapping: RuleMappingArgs,
    },
}

#[derive(Args)]
struct RuleMappingArgs {
    #[arg(long, requires_all = ["rule", "justification", "expiry"])]
    scanner: Option<String>,
    #[arg(long, requires = "scanner")]
    rule: Option<String>,
    #[arg(long, requires = "scanner")]
    justification: Option<String>,
    #[arg(long, value_name = "YYYY-MM-DD", requires = "scanner")]
    expiry: Option<String>,
}

#[derive(Clone, ValueEnum)]
enum Mode {
    Cli,
    Tui,
    Mcp,
}

pub fn run() -> ExitCode {
    let cli: Cli = Cli::parse();
    if let Some(Command::Create { entry }) = cli.command {
        if !matches!(cli.mode, Mode::Cli) || cli.ssp.is_some() {
            eprintln!("Expected CLI mode without --ssp for create");
            return ExitCode::FAILURE;
        }
        let (mappings_file, result) = match entry {
            CreateEntry::Component {
                mappings_file,
                component_name,
            } => {
                let result = mappings::create_component(&mappings_file, &component_name);
                (mappings_file, result)
            }
            CreateEntry::Mapping {
                mappings_file,
                component_name,
                control,
                rule_mapping,
            } => {
                let result = match rule_mapping {
                    RuleMappingArgs {
                        scanner: Some(scanner),
                        rule: Some(rule),
                        justification: Some(justification),
                        expiry: Some(expiry),
                    } => mappings::create_rule_mapping(
                        &mappings_file,
                        &component_name,
                        &control,
                        &scanner,
                        &rule,
                        &justification,
                        &expiry,
                    ),
                    _ => {
                        mappings::create_control_mapping(&mappings_file, &component_name, &control)
                    }
                };
                (mappings_file, result)
            }
        };
        if let Err(error) = result {
            eprintln!("Failed to update {}: {error}", mappings_file.display());
            return ExitCode::FAILURE;
        }
        return ExitCode::SUCCESS;
    }
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
