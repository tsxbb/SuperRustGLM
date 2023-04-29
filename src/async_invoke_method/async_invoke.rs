mod history_message;

extern crate toml;

use std::error::Error;
use std::time::Duration;
use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
us