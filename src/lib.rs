
mod custom_jwt;
mod api_operation;
mod async_invoke_method;
mod sync_invoke_method;
mod sse_invoke_method;

use std::collections::HashMap;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use regex::Regex;

#[derive(Debug)]
pub struct RustGLM {
    chatglm_response: String,
    chatglm_input: String,
}

enum CallResult {
    Success(String),
    Error(String),
}

impl RustGLM {
    pub async fn new() -> Self {
        RustGLM {
            chatglm_response: String::new(),
            chatglm_input: String::new(),
        }
    }

    pub fn set_user_input(&mut self, input: String) {
        self.chatglm_input = input;
    }

    async fn async_invoke_calling(jwt_token: &str, user_input: &str, glm_version: &str, user_config: &str) -> String {
        let jwt_token_clone = jwt_token.to_string();
        let user_input_clone = user_input.to_string();
        let glm_version_clone = glm_version.to_string();
        let user_config_clone = user_config.to_string();

        let handle = tokio::spawn(async move {
            let response =
                async_invoke_method::ReceiveAsyncInvokeOnlyText::new(&jwt_token_clone, &user_input_clone, &glm_version_clone, user_config_clone);
            response
                .await
                .get_response()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Error getting response.".to_string())
        });

        handle.await.expect("Failed to await JoinHandle")
    }

    async fn sync_invoke_calling(jwt_token: &str, user_input: &str, glm_version: &str, user_config: &str) -> String {
        let sync_call = sync_invoke_method::ReceiveInvokeModelOnlyText::new(jwt_token, user_input, glm_version, user_config);
