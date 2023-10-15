use rsntp::SntpClient;
use chrono::{DateTime, Local};

pub fn time_sync() -> i64 {
    let clien