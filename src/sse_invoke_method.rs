mod sse_invoke;

#[derive(Debug)]
pub struct ReceiveSSEInvokeModelOnlyText {
    response_sse_message: Option<String>,
    default_url: String,
}

impl ReceiveSSEInvokeModelOnly