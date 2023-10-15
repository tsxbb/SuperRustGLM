use rsntp::SntpClient;
use chrono::{DateTime, Local};

pub fn time_sync() -> i64 {
    let client = SntpClient::new();
    let result = client.synchronize("ntp.aliyun.com").unwr