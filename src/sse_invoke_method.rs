mod sse_invoke;

#[derive(Debug)]
pub struct ReceiveSSEInvokeModelOnlyText {
    response_sse_message: Option<String>,
    default_url: String,
}

impl ReceiveSSEInvokeModelOnlyText {
    pub async fn new(token: &str, message: &str, glm_version: &str, user_co