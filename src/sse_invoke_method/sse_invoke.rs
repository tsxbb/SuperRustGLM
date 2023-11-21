mod history_message;

extern crate toml;

use std::fs::File;
use std::collections::VecDeque;
use std::error::Error;
use std::io::Read;
use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use futures::stream::StreamExt;

lazy_static::lazy_static! {
    static ref UNICODE_REGEX: regex::Regex = regex::Regex::new(r"\\u[0-9a-fA-F]{4}").unwrap();
}

/*
ChatGLM-3, ChatGLM-4 Config
*/

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
struct SSEConfig {
    ai_config_glm3: Vec<AiResponse>,
    ai_config_glm4: Vec<AiResponse>,
}

fn sse_read_config(file_path: &str, glm: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    let config: SSEConfig = toml::from_str(&file_content)?;

    let response = match glm {
        "glm-3" => &config.ai_config_glm3,
        "glm-4" => &config.ai_config_glm4,
        _ => return Err("Invalid glm-format".into()),
    };

    serde_json::to_string(response).map_err(Into::into)
}


/*
ChatGLM-4V Config
*/

#[derive(Serialize, Deserialize, Debug)]
struct Glm4vConfig {
    model: Option<String>,
    user_role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GLM4VConfig {
    ai_config_glm4v: Vec<Glm4vConfig>,
}

async fn glm4v_read_config(file_path: &str, glm: &str) -> Result<String, Box<dyn Error>> {
    let file_content = tokio::fs::read_to_string(file_path).await?;
    let config: GLM4VConfig = toml::from_str(&file_content)?;

    let response = match glm {
        "glm-4v" => config.ai_config_glm4v,
        _ => return Err("Invalid glm4v".into()),
    };

    let json_string = serde_json::to_string(&response)?;

    Ok(json_string)
}


/*
Create chatglm-4v message format by Regex
*/


#[derive(Serialize, Deserialize)]
struct ImageUrl {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct Content {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<ImageUrl>,
}

#[derive(Serialize, Deserialize)]
struct JSONResonseData {
    role: String,
    content: Vec<Content>,
}

fn create_4vjson_message(user_role: String, user_input: String) -> JSONResonseData {
    let regex_input = Regex::new(r"([^@]+)@([^@]+)").unwrap();

    let mut part1_content = String::new();
    let mut part2_content = String::new();

    if let Some(captures_content) = regex_input.captures(&user_input) {
        if let Some(first_part) = captures_content.get(1) {
            part1_content = first_part.as_str().to_string();
        }
        if let Some(second_part) = captures_content.get(2) {
            part2_content = second_part.as_str().to_string();
        }
    } else {
        println!("Input does not match the pattern");
    }

    JSONResonseData {
        role: user_role,
        content: vec![
            Content {
                content_type: "text".to_string(),
                text: Some(part1_content),
                image_url: None,
            },
            Content {
                content_type: "image_url".to_string(),
                text: None,
                image_url: Some(ImageUrl { url: part2_content }),
            },
        ],
    }
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
        let input_message = sel