#![allow(missing_docs)]
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SolarVal{
    Data(String),
    Value(i32),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SolarData {
    pub key: String,
    pub val: SolarVal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Solar {
    pub err: u8,
    pub desc: String,
    pub dat: Vec<SolarData>,
}
