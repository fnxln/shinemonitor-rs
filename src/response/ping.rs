#![allow(missing_docs)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimplePing {
    pub err: u8,
    pub desc: String,
}
