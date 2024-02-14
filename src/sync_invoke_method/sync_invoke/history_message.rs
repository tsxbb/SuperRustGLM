use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const HISTORY_FILE: &str = "chatglm_history.json";

pub struct HistoryMessage {
    history_file_path: String,
}

impl HistoryMessage {
    pub