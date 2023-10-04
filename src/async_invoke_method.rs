mod async_invoke;

use crate::async_invoke_method::async_invoke::AsyncInvokeModel;

pub struct ReceiveAsyncInvokeOnlyText {
    response_async_message: Option<String>,
    default_url: String,
    async_invoke_check_url: String,
}

impl ReceiveAsyncInvokeOnlyText {
    pub async fn new(token: &str, message: &str, glm_version:&str, user_config: String) -> Self {
        let default_url = "https://open.bigmodel.cn/api/paas/