#![allow(missing_docs)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PidInfo {
    pub uid: u32,
    pub usr: String,
    pub pid: u32,
    pub pname: String,
    pub status: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PidDat {
    pub total: u8,
    pub page: u8,
    pub pagesize: u8,
    pub info: Vec<PidInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pid {
    pub err: u8,
    pub desc: String,
    pub dat: PidDat,
}
