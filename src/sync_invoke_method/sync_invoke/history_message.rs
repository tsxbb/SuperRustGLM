use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const HISTORY_FILE: &str = "chatglm_history.json";

pub struct Histo