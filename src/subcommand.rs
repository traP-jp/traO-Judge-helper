use std::path::PathBuf;

use crate::LanguageSettings;
use anyhow::{Context, Result};
use clap::ArgMatches;

pub fn run_sub_command(
    subcommand_name: &str,
    subcommand_args: &ArgMatches,
    language_settings: LanguageSettings,
) -> Result<()> {
    if subcommand_name == "run" {
        run(subcommand_args, language_settings).context("Command phase failed")
    } else if subcommand_name == "compile" {
        compile(subcommand_args, language_settings).context("Run phase failed")
    } else {
        Err(anyhow::anyhow!("Unknown subcommand: {}", subcommand_name))
    }
}

fn compile(subcommand_args: &ArgMatches, language_settings: LanguageSettings) -> Result<()> {
    let source_file = subcommand_args
        .get_one::<PathBuf>("source")
        .context("Failed to get the source file")?;
    let dest_dir = subcommand_args
        .get_one::<PathBuf>("dest_dir")
        .context("Failed to get the destination directory")?;
    let output = std::process::Command::new(language_settings.compile)
        .env("SRC", source_file)
        .env("DEST", dest_dir)
        .output()
        .context("Failed to execute compile command")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Compile command failed with status: {}",
            output.status
        ));
    }
    Ok(())
}

fn run(subcommand_args: &ArgMatches, language_settings: LanguageSettings) -> Result<()> {
    let dest_dir = subcommand_args
        .get_one::<PathBuf>("dest_dir")
        .context("Failed to get the destination directory")?;
    let input_file = subcommand_args
        .get_one::<PathBuf>("input")
        .context("Failed to get the input file")?;
    let output_file = subcommand_args
        .get_one::<PathBuf>("output")
        .context("Failed to get the output file")?;
    let output = std::process::Command::new(language_settings.run)
        .env("DEST", dest_dir)
        .env("INPUT", input_file)
        .env("OUTPUT", output_file)
        .output()
        .context("Failed to execute run command")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Run command failed with status: {}",
            output.status
        ));
    }
    Ok(())
}
