mod history_message;

extern crate toml;

use std::error::Error;
use std::time::Duration;
use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::time::sleep;


#[derive(Serialize, Deserialize, Debug)]
struct AiResponse {
    language_model: Option<String>,
    system_role: Option<String>,
    system_content: Option<String>,
    user_role: Option<String>,
    assistant_role: Option<String>,
    max_tokens: Option<f64>,
    temp_float: Option<f64>,
    top_p_float: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AiConfig {
    ai_config_glm3: Vec<AiResponse>,
    ai_config_glm4: Vec<AiResponse>,
}

async fn async_read_config(file_path: &str, glm: &str) -> Result<String, Box<dyn Error>> {
    let file_content = tokio::fs::read_to_string(file_path).await?;
    let config: AiConfig = toml::from_str(&file_content)?;

    let response = mat