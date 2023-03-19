use std::error::Error;
use once_cell::sync::OnceCell;
use std::io::BufRead;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct ChatApiConfig {
    api_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AiConfig {
    chatglm_api_key: Vec<ChatApiConfig>,
}

pub async fn chatglm_api_read_config(file_path: &str, glm: &str) -> Result<String, Box<dyn Error>> {
    let file_content = tokio::fs::read_to_string(file_path).await?;
    let config: AiConfig = toml::from_str(&file_content)?;

    let response = match glm {
        "chat