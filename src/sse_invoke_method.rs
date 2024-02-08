mod sse_invoke;

#[derive(Debug)]
pub struct ReceiveSSEInvokeModelOnlyText {
    response_sse_message: Option<String>,
    default_url: String,
}

impl ReceiveSSEInvokeModelOnlyText {
    pub async fn new(token: &str, message: &str, glm_version: &str, user_config: &str) -> Self {
        let default_url = "https://open.bigmodel.cn/api/paas/v4/chat/completions".trim().to_string();

        let mut instance = Self {
            response_sse_message: None,
            default_url,
        };

        instance.send_request_and_wait(token, message, glm_version, user_config).await;