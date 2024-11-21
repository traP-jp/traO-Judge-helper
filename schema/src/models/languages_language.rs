/*
 * traO-Judge language settings schema
 *
 * Schema only (no endpoint provided)
 *
 * The version of the OpenAPI document: 0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LanguagesLanguage {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "binName")]
    pub bin_name: String,
    #[serde(rename = "compile")]
    pub compile: String,
    #[serde(rename = "run")]
    pub run: String,
    #[serde(rename = "libraries", skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<models::LanguagesLibrary>>,
}

impl LanguagesLanguage {
    pub fn new(name: String, bin_name: String, compile: String, run: String) -> LanguagesLanguage {
        LanguagesLanguage {
            name,
            bin_name,
            compile,
            run,
            libraries: None,
        }
    }
}

