use serde_json::json;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

const HISTORY_FILE: &str = "chatglm_history.json";

pub struct HistoryMessage {
    history_file_p