mod cleaner;
mod language;
mod processor;

use anyhow::Result;
use clap::Parser;
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "cclean",
    version = "0.1.0",
    about = "Code comment cleaning tool - supports C/C++, Java/JavaScript/TypeScript, Python, HTML/CSS",
    long_about = "A powerful command-line tool for automatically removing comments from code in multiple programming languages"
)]
struct Cli {
    #[clap(
        value_name = "PATH",
        help = "File or directory path to process"
    )]
    path: PathBuf,

    #[clap(
        short = 'o',
        long = "output",
        value_name = "OUTPUT",
        help = "Output file path (single file mode) or output directory (directory mode)"
    )]
    output: Option<PathBuf>,

    #[clap(
        short = 'r',
        long = "recursive",
        help = "Recursively process all files in directory"
    )]
    recursive: bool,

    #[clap(
        short = 'i',
        long = "in-place",
        help = "Modify original file directly (no backup)"
    )]
    in_place: bool,

    #[clap(
        short = 'b',
        long = "backup",
        help = "Create backup file (.bak)"
    )]
    backup: bool,

    #[clap(
        short = 'l',
        long = "lang",
        value_name = "LANGUAGE",
        help = "Manually specify programming language (c, cpp, java, js, ts, python, html, css)"
    )]
    language: Option<String>,

    #[clap(
        short = 'v',
        long = "verbose",
        help = "Show detailed information"
    )]
    verbose: bool,

    #[clap(
        long = "dry-run",
        help = "Dry run, do not actually modify files"
    )]
    dry_run: bool,

    #[clap(
        short = 'e',
        long = "extensions",
        value_name = "EXT",
        help = "Specify file extensions to process (comma-separated, e.g.: c,cpp,h)"
    )]
    extensions: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("{}", "Code Comment Cleaning Tool v0.1.0".cyan().bold());
        println!("{}", "=".repeat(50).cyan());
    }

    processor::process(&cli.path, &cli)?;

    if cli.verbose {
        println!("{}", "\nProcessing completed!".green().bold());
    }

    Ok(())
}

pub fn get_cli_config() -> CliConfig {
    let cli = Cli::parse();
    CliConfig {
        output: cli.output,
        recursive: cli.recursive,
        in_place: cli.in_place,
        backup: cli.backup,
        language: cli.language,
        verbose: cli.verbose,
        dry_run: cli.dry_run,
        extensions: cli.extensions,
    }
}

pub struct CliConfig {
    pub output: Option<PathBuf>,
    pub recursive: bool,
    pub in_place: bool,
    pub backup: bool,
    pub language: Option<String>,
    pub verbose: bool,
    pub dry_run: bool,
    pub extensions: Option<String>,
}

impl From<&Cli> for CliConfig {
    fn from(cli: &Cli) -> Self {
        CliConfig {
            output: cli.output.clone(),
            recursive: cli.recursive,
            in_place: cli.in_place,
            backup: cli.backup,
            language: cli.language.clone(),
            verbose: cli.verbose,
            dry_run: cli.dry_run,
            extensions: cli.extensions.clone(),
        }
    }
}
