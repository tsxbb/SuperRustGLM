mod async_invoke;

use crate::async_invoke_method::async_invoke::AsyncInvokeModel;

pub struct ReceiveAsyncInvokeOnlyText {
    response_async_message: Option<String>,
    default_url: String,
    async_invoke_check_url: String,
}

impl ReceiveAsyncInvokeOnlyText {
    pu