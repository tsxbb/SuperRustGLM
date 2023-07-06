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
pub struct AsyncInvokeModel {
    get_message: String,
    search_task_id: String,
}

impl AsyncInvokeModel {
    pub fn new() -> Self {
        AsyncInvokeModel {
            get_message: String::new(),
            search_task_id: String::new(),
        }
    }

    pub async fn async_request(token: String, input: String, glm_version:&str,user_config: String, default_url: String, check_url: String) -> Result<String, Box<dyn Error>> {
        let mut async_invoke_model = Self::new();
        Self::async_invoke_request_method(&mut async_invoke_model, token.clone(), input.clone(), glm_version,user_config.clone(), default_url.clone()).await?;
        let search_id = async_invoke_model.search_task_id.clone();
        let response_data = Self::wait_for_task_to_complete(&*search_id.clone(), &*token.clone(), &*check_url.clone()).await?;
        let result = async_invoke_model.process_task_status(&response_data, &input);
        Ok(result)
    }

    async fn generate_json_request_body(
        language_model: &str,
        system_role: &str,
        system_content: &str,
        user_role: &str,
        user_input: &str,
        max_token: f64,
        temp_float: f64,
        top_p_float: f64,
    ) -> Result<String, Box<dyn Error>> {
        let message_process = MessageProcessor::new();

        //Debug for reading history from file
        /*
        if let Some(input_message) = message_process.set_input_message() {
            println!("Loaded history from file:\n{}", input_message);
        } else {
            println!("No history available.");
        }*/

        let messages = json!([
        {"role": system_role, "content": system_content},
        {"role": user_role, "content": message_process.last_messages(user_role,user_input)}
    ]);

        let json_request_body = json!({
        "model": language_model,
        "messages": messages,
        "stream": false,
        "max_tokens":max_token,
        "temperature": temp_float,
        "top_p": top_p_float
    });

        let json_string = serde_json::to_string(&json_request_body)?;

        // 替换字符，注意使用转义符号
        let result = json_string.replace(r"\\\\", r"\\").replace(r"\\", r"").trim().to_string();

        //