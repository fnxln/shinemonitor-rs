#![allow(missing_docs)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthDat {
    pub secret: String,
    pub expire: u32,
    pub token: String,
    pub role: u8,
    pub usr: String,
    pub uid: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub err: u8,
    pub desc: String,
    pub dat: AuthDat,
}
