mod config;
mod subcommand;
use crate::config::*;
use crate::subcommand::*;
use clap::{Arg, Command};
use std::path::PathBuf;

fn main() {
    let command = Command::new("toj-helper")
        .version("0.1.0")
        .about("A helper for the TOJ")
        .arg(
            Arg::new("configjson")
                .short('c')
                .long("configjson")
                .value_name("CONFIG_FILE")
                .required(true),
        )
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .value_name("LANGUAGE")
                .required(true),
        )
        .subcommand(
            Command::new("compile")
                .about("Compile the source code")
                .arg(
                    Arg::new("source")
                        .required(true)
                        .index(1)
                        .value_name("SOURCE_FILE")
                        .required(true),
                )
                .arg(
                    Arg::new("dest_dir")
                        .short('d')
                        .long("dest-dir")
                        .value_name("DEST_DIR")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("run")
                .about("Run the compiled code")
                .arg(
                    Arg::new("dest_dir")
                        .short('d')
                        .long("dest-dir")
                        .value_name("DEST_DIR")
                        .required(true),
                )
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("INPUT_FILE")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT_FILE")
                        .required(true),
                ),
        )
        .subcommand_required(true);

    // Parse the command line arguments
    let matches = command.get_matches();
    let configjson_path: &PathBuf = matches
        .get_one::<PathBuf>("configjson")
        .expect("configjson is required");
    let language_settings =
        get_language_settings(configjson_path).expect("Failed to get language settings");
    let language: &String = matches
        .get_one::<String>("language")
        .expect("language is required");
    let language_setting = language_settings
        .get(language)
        .expect(&format!("Language setting for '{}' not found", language));

    // Run the subcommand
    match matches.subcommand() {
        Some((name, subcommand_matches)) => {
            run_sub_command(name, subcommand_matches, language_setting.clone())
                .expect("Failed to run subcommand");
        }
        None => {
            println!("No subcommand specified");
        }
    }
}
