use anyhow::{Context, Result};
use schema::models::*;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn get_language_settings(
    config_json_path: &PathBuf,
) -> Result<HashMap<String, LanguageSettings>> {
    let config_json =
        std::fs::read_to_string(config_json_path).context("Failed to read the config json file")?;
    let config = serde_json::from_str::<Languages>(&config_json)
        .context("Failed to parse the config json file")?;
    let mut language_settings = HashMap::new();
    for language in config.languages {
        let language_name = language.name;
        let compile_command = language.compile;
        let run_command = language.run;
        language_settings.insert(
            language_name,
            LanguageSettings {
                compile: compile_command,
                run: run_command,
            },
        );
    }
    Ok(language_settings)
}

#[derive(Clone, Debug)]
pub struct LanguageSettings {
    pub compile: String,
    pub run: String,
}
