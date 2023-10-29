
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

        match sync_call.await.get_response_message() {
            Some(message) => message.to_string(), // Return the message as String
            None => "Error: Unable to get sync response.".to_string(),
        }
    }

    async fn sse_invoke_calling(jwt_token: &str, user_input: &str, glm_version: &str, user_config: &str) -> String {
        let sse_call = sse_invoke_method::ReceiveSSEInvokeModelOnlyText::new(jwt_token, user_input, glm_version, user_config);

        match sse_call.await.get_response_message() {
            Some(message) => message.to_string(), // Return the message as String
            None => "Error: Unable to get SSE response.".to_string(),
        }
    }


    async fn call_sse(jwt: Arc<String>, user_in: &str, user_glm_version: &str, user_config: &str) -> String {
        Self::sse_invoke_calling(&jwt, user_in, user_glm_version, user_config).await
    }


    async fn call_sync(jwt: Arc<String>, user_in: &str, user_glm_version: &str, user_config: &str) -> String {
        Self::sync_invoke_calling(&jwt, user_in, user_glm_version, user_config).await
    }

    async fn call_async(jwt: Arc<String>, user_in: &str, user_glm_version: &str, user_config: &str) -> String {
        Self::async_invoke_calling(&jwt, user_in, user_glm_version, user_config).await
    }

    async fn regex_checker(regex: &Regex, input: &str) -> bool {
        regex.is_match(&*input)
    }


    async fn is_call_valid(
        part1_content: String,
        part2_content: Arc<String>,
        glm_version: Arc<String>,
        user_config: Arc<String>,
        jwt: Arc<String>,
    ) -> CallResult {
        let mut methods: HashMap<&str, Box<dyn Fn() -> BoxFuture<'static, String> + Send>> =
            HashMap::new();
        let jwt_for_sse = Arc::clone(&jwt);
        let jwt_for_async = Arc::clone(&jwt);
        let jwt_for_sync = Arc::clone(&jwt);

        let user_config_sse = Arc::clone(&user_config);
        let user_config_async = Arc::clone(&user_config);
        let user_config_sync = Arc::clone(&user_config);

        let glm_version_sse = Arc::clone(&glm_version);
        let glm_version_async = Arc::clone(&glm_version);
        let glm_version_sync = Arc::clone(&glm_version);

        let part2_content_sse = Arc::clone(&part2_content);
        let part2_content_async = Arc::clone(&part2_content);
        let part2_content_sync = Arc::clone(&part2_content);

        methods.insert("sse", Box::new(move || {
            let jwt_for_sse = Arc::clone(&jwt_for_sse);
            let part2_content = Arc::clone(&part2_content_sse);
            let user_glm_version = Arc::clone(&glm_version_sse);
            let user_config = Arc::clone(&user_config_sse);
            async move {
                RustGLM::call_sse(jwt_for_sse, part2_content.trim(), &user_glm_version, &user_config).await
            }
                .boxed()
        }));

        methods.insert("async", Box::new(move || {
            let jwt_for_async = Arc::clone(&jwt_for_async);
            let part2_content = Arc::clone(&part2_content_async);
            let user_glm_version = Arc::clone(&glm_version_async);
            let user_config = Arc::clone(&user_config_async);
            async move {