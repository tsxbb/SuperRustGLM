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

    let response = match glm {
        "glm-3" => config.ai_config_glm3,
        "glm-4" => config.ai_config_glm4,
        _ => return Err("Invalid glm4v".into()),
    };

    let json_string = serde_json::to_string(&response)?;

    Ok(json_string)
}

pub struct MessageProcessor {
    messages: history_message::HistoryMessage,
}

impl MessageProcessor {
    pub fn new() -> Self {
        MessageProcessor {
            messages: history_message::HistoryMessage::new(),
        }
    }

    pub fn set_input_message(&self) -> Option<String> {
        let message = self.messages.load_history_from_file();
        if !message.is_empty() {
            Some(message)
        } else {
            None
        }
    }

    pub fn last_messages(&self, role: &str, messages: &str) -> String {
        let input_message = self.set_input_message().unwrap_or_default();

        let mut input: Value = serde_json::from_str(&input_message).unwrap_or_default();
        input["rol