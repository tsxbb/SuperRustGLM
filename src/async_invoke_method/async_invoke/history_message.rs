use serde_json::json;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

const HISTORY_FILE: &str = "chatglm_history.json";

pub struct HistoryMessage {
    history_file_path: PathBuf,
}

impl HistoryMessage {
    pub fn new() -> Self {
        let history_file_path = PathBuf::from(HISTORY_FILE);
        Self::create_history_file_if_not_exists(&history_file_path);

        HistoryMessage { history_file_path }
    }

    fn create_history_file_if_not_exists(file_path: &Path) {
        if !file_path.exists() {
            if let Err(err) = File::create(