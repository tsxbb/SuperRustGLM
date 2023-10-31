mod history_message;

extern crate toml;

use std::fs::File;
use std::collections::VecDeque;
use std::error::Error;
use std::io::Read;
use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use futures::stream::StreamExt;

lazy_static: