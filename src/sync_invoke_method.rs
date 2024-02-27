mod sync_invoke;

#[derive(Debug)]
pub struct ReceiveInvokeModelOnlyText {
    response_sync_message: Option<String>,
    image_url: String,
    default_url: String,
}

impl ReceiveInvokeModelOnlyText {
    pub async fn new(token: &str, message: &str, glm_version: &str, user_config: 