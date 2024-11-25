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
pub struct DummyGet200Response {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DummyGet200Response {
    pub fn new() -> DummyGet200Response {
        DummyGet200Response {
            message: None,
        }
    }
}

