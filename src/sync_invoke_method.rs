mod sync_invoke;

#[derive(Debug)]
pub struct ReceiveInvokeModelOnlyText {
    response_sync_message: Option<String>,
    image_url: String,
    default_url: String,
}

impl ReceiveInvokeModelOnlyTex