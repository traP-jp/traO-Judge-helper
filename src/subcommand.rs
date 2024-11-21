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
        run(subcommand_args, language_settings).context("Run phase failed")
    } else if subcommand_name == "compile" {
        compile(subcommand_args, language_settings).context("Compile phase failed")
    } else {
        Err(anyhow::anyhow!("Unknown subcommand: {}", subcommand_name))
    }
}

fn compile(subcommand_args: &ArgMatches, language_settings: LanguageSettings) -> Result<()> {
    let source_file_string = subcommand_args
        .get_one::<String>("source")
        .context("Failed to get the source file")?;
    let source_file = PathBuf::from(source_file_string);
    let dest_dir_string = subcommand_args
        .get_one::<String>("dest_dir")
        .context("Failed to get the destination directory")?;
    let dest_dir = PathBuf::from(dest_dir_string);
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(language_settings.compile)
        .env("SOURCE", source_file)
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
    let dest_dir_string = subcommand_args
        .get_one::<String>("dest_dir")
        .context("Failed to get the destination directory")?;
    let dest_dir = PathBuf::from(dest_dir_string);
    let input_file_string = subcommand_args
        .get_one::<String>("input")
        .context("Failed to get the input file")?;
    let input_file = PathBuf::from(input_file_string);
    let output_file_string = subcommand_args
        .get_one::<String>("output")
        .context("Failed to get the output file")?;
    let output_file = PathBuf::from(output_file_string);
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(language_settings.run)
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
