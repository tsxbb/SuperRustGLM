
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
