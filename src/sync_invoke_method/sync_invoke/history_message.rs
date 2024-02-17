use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const HISTORY_FILE: &str = "chatglm_history.json";

pub struct HistoryMessage {
    history_file_path: String,
}

impl HistoryMessage {
    pub fn new() -> Self {
        let history_file_path = String::from(HISTORY_FILE);
        Self::create_history_file_if_not_exists(&history_file_path);

        HistoryMessage { history_file_path }
    }

    fn create_history_file_if_not_exists(file_path: &str) {
        let path = Path::new(file_path);

        if !path.exists() {
            if let Err(err) = File::create(file_path) {
                eprintln!("Failed to create history file: {}", err)