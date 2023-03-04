use std::error::Error;
use once_cell::sync::OnceCell;
use std::io::BufRead;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug