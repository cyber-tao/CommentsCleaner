use crate::cleaner;
use crate::language::Language;
use crate::Cli;
use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn process(path: &Path, cli: &Cli) -> Result<()> {
    if path.is_file() {
        process_file(path, cli)?;
    } else if path.is_dir() {
        process_directory(path, cli)?;
    } else {
        anyhow::bail!("Path does not exist: {}", path.display());
    }
    Ok(())
}

fn process_directory(dir: &Path, cli: &Cli) -> Result<()> {
    let extensions: Option<Vec<String>> = cli.extensions.as_ref().map(|exts| {
        exts.split(',')
            .map(|s| s.trim().to_string())
            .collect()
    });

    let walker = if cli.recursive {
        WalkDir::new(dir).into_iter()
    } else {
        WalkDir::new(dir).max_depth(1).into_iter()
    };

    let mut processed_count = 0;
    let mut skipped_count = 0;

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        
        if !path.is_file() {
            continue;
        }

        if let Some(ref exts) = extensions {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if !exts.contains(&ext.to_string()) {
                    skipped_count += 1;
                    continue;
                }
            } else {
                skipped_count += 1;
                continue;
            }
        }

        let language = if let Some(ref lang_str) = cli.language {
            Language::from_str(lang_str)
        } else {
            Language::from_path(path)
        };

        if language.is_none() {
            if cli.verbose {
                println!(
                    "{} {}",
                    "Skipping:".yellow(),
                    path.display().to_string().dimmed()
                );
            }
            skipped_count += 1;
            continue;
        }

        match process_single_file(path, cli, language.unwrap()) {
            Ok(_) => {
                processed_count += 1;
            }
            Err(e) => {
                eprintln!("{} {}: {}", "Error:".red(), path.display(), e);
            }
        }
    }

    if cli.verbose {
        println!(
            "\n{} {} files, {} {} files",
            "Processed".green(),
            processed_count,
            "skipped".yellow(),
            skipped_count
        );
    }

    Ok(())
}

fn process_file(file: &Path, cli: &Cli) -> Result<()> {
    let language = if let Some(ref lang_str) = cli.language {
        Language::from_str(lang_str)
            .context("Cannot recognize the specified programming language")?
    } else {
        Language::from_path(file)
            .context("Cannot recognize programming language from file extension, please use -l parameter to specify manually")?
    };

    process_single_file(file, cli, language)
}

fn process_single_file(file: &Path, cli: &Cli, language: Language) -> Result<()> {
    if cli.verbose {
        println!(
            "{} {} ({})",
            "Processing:".cyan(),
            file.display(),
            language.name().yellow()
        );
    }

    let content = fs::read_to_string(file)
        .context(format!("Cannot read file: {}", file.display()))?;

    let cleaned_content = cleaner::clean_comments(&content, language);

    if cli.dry_run {
        if cli.verbose {
            println!("{}", "  [Dry run - file not modified]".dimmed());
        }
        return Ok(());
    }

    let output_path = determine_output_path(file, cli)?;

    if cli.backup && cli.in_place {
        let backup_path = file.with_extension(
            format!(
                "{}.bak",
                file.extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
            )
        );
        fs::copy(file, &backup_path)
            .context(format!("Cannot create backup file: {}", backup_path.display()))?;
        
        if cli.verbose {
            println!("  {} {}", "Backup:".green(), backup_path.display());
        }
    }

    fs::write(&output_path, cleaned_content)
        .context(format!("Cannot write file: {}", output_path.display()))?;

    if cli.verbose {
        println!("  {} {}", "Output:".green(), output_path.display());
    }

    Ok(())
}

fn determine_output_path(file: &Path, cli: &Cli) -> Result<PathBuf> {
    if cli.in_place {
        Ok(file.to_path_buf())
    } else if let Some(ref output) = cli.output {
        if cli.recursive && output.is_dir() {
            let file_name = file.file_name()
                .context("Cannot get filename")?;
            Ok(output.join(file_name))
        } else {
            Ok(output.clone())
        }
    } else {
        let parent = file.parent().unwrap_or_else(|| Path::new("."));
        let stem = file.file_stem()
            .and_then(|s| s.to_str())
            .context("Cannot get filename")?;
        let extension = file.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        
        let output_name = if extension.is_empty() {
            format!("{}_cleaned", stem)
        } else {
            format!("{}_cleaned.{}", stem, extension)
        };
        
        Ok(parent.join(output_name))
    }
}
