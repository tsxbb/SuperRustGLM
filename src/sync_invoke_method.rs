mod sync_invoke;

#[derive(Debug)]
pub struct ReceiveInvokeModelOnlyText {
    response_sync_message: Option<String>,
    image_url: String,
    default_url: String,
}

impl ReceiveInvokeModelOnlyText {
    pub async fn new(token: &str, message: &str, glm_version: &str, user_config: &str) -> Self {
        let image_url = "https://open.bigmodel.cn/api/paas/v4/images/generations".trim().to_string();
        let default_url = "https://open.big