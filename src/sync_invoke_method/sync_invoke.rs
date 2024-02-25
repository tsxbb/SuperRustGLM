
mod history_message;

extern crate toml;

use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

fn sync_read_config(file_path: &str, glm: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    let config: AiConfig = toml::from_str(&file_content)?;

    let response = match glm {
        "glm-3" => config.ai_config_glm3,
        "glm-4" => config.ai_config_glm4,
        _ => return Err(Box::from("Invalid glm")),
    };

    // 将 AiResponse 向量转换为 JSON 字符串
    let json_string = serde_json::to_string(&response)?;

    Ok(json_string)
}

/*
ChatGLM-CogView Config
*/

#[derive(Serialize, Deserialize, Debug)]
struct CogView {
    model: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CogViewConfig3 {
    ai_cogview_config_3: Vec<CogView>,
}

fn cogview_read_config(file_path: &str, glm: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    let config: CogViewConfig3 = toml::from_str(&file_content)?;

    let response = match glm {
        "cogview-3" => config.ai_cogview_config_3,
        _ => return Err(Box::from("Invalid glm")),
    };

    let json_string = serde_json::to_string(&response)?;

    Ok(json_string)
}



/*
History Message Controller(Save Messages)
*/
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
        input["role"] = Value::String(role.to_string());
        input["content"] = Value::String(messages.to_string());

        let texts = serde_json::to_string(&input).unwrap_or_default();

        let regex = Regex::new(r",(\s*})").expect("Failed to create regex pattern");

        let user_messages = input_message.clone() + &texts.clone();
        let result = regex.replace_all(&user_messages, "");

        result.to_string()
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SyncInvokeModel {
    get_message: String,
    ai_response_data: String,
    fetch_drawer: String,
}

impl SyncInvokeModel {
    pub fn new() -> Self {
        SyncInvokeModel {
            get_message: String::new(),
            ai_response_data: String::new(),
            fetch_drawer: String::new(),
        }
    }

    pub async fn sync_request(token: String, input: String, glm_version: &str, user_config: &str, iamge_url: String, default_url: String) -> Result<String, Box<dyn Error>> {
        let mut sync_invoke_model = Self::new();
        Self::sync_invoke_request_method(&mut sync_invoke_model, token.clone(), input.clone(), glm_version, user_config, iamge_url.clone(), default_url.clone()).await?;
        let response_message = sync_invoke_model.ai_response_data.clone();
        let result = sync_invoke_model.choose_task_status(&*response_message, &input).await;
        Ok(result)
    }


    /*
    cogview request body by JSON
     */
    async fn generate_cogview_request_body(