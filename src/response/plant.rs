#![allow(missing_docs)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlantAddress {
    pub country: String,
    pub province: String,
    pub city: String,
    pub address: String,
    pub lon: String,
    pub lat: String,
    pub timezone: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlantProfit {
    #[serde(rename = "unitProfit")]
    pub unit_profit: String,
    pub currency: String,
    pub coal: String,
    pub co2: String,
    pub so2: String,
    #[serde(rename = "soldProfit")]
    pub sold_profit: f32,
    #[serde(rename = "selfProfit")]
    pub self_profit: f32,
    #[serde(rename = "purchProfit")]
    pub purch_profit: f32,
    #[serde(rename = "consProfit")]
    pub cons_profit: f32,
    #[serde(rename = "feedProfit")]
    pub feed_profit: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlantDat {
    pub pid: u32,
    pub uid: u32,
    pub name: String,
    pub status: u8,
    #[serde(rename = "energyOffset")]
    pub energy_offset: f32,
    pub address: PlantAddress,
    pub profit: PlantProfit,
    #[serde(rename = "nominalPower")]
    pub nominal_power: String,
    #[serde(rename = "energyYearEstimate")]
    pub energy_year_estimate: String,
    #[serde(rename = "designCompany")]
    pub design_company: String,
    #[serde(rename = "picBig")]
    pub pic_big: String,
    #[serde(rename = "picSmall")]
    pub pic_small: String,
    pub install: String,
    pub gts: String,
    pub flag: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plant {
    pub err: u8,
    pub desc: String,
    pub dat: PlantDat,
}
