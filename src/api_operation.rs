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
        "chatglm_api_key" => config.chatglm_api_key,
        _ => return Err("Invalid ChatGLM API".into()),
    };

    let json_string = serde_json::to_string(&response)?;

    Ok(json_string)
}

pub struct APIKeys {
    user_id: String,
    user_secret: String,
}

impl APIKeys {
    fn new(user_id: &str, user_secret: &str) -> APIKeys {
        APIKeys {
            user_id: user_id.to_string(),
            user_secret: user_secret.to_string(),
        }
    }

    pub fn get_instance(api: &str) -> &APIKeys {
        static INSTANCE: OnceCell<APIKeys> = OnceCell::new();

        INSTANCE.get_or_init(|| {
            let parts: Vec<&str> = api.trim().split('.').collect();
            if parts.len() == 2 {
                APIKeys::new(parts[0], parts[1])
            } else {
                panic!("Your API Key is Invalid");
            }
        })
    }

    pub fn g