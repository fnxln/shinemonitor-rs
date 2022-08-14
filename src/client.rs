const COMPANY_KEY: &str = "bnrl_frRFjEz8Mkn";
const BASE_URL: &str = "https://web.shinemonitor.com/public";

use crate::response::auth::{Auth, AuthDat};
use crate::response::pid::Pid;
use crate::response::ping::SimplePing;
use crate::response::plant::Plant;
use crate::response::solar::Solar;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use reqwest::Error;
use serde::de::DeserializeOwned;
use std::time::{SystemTime, UNIX_EPOCH};

/// ShineMonitor Client
pub struct ShineMonitorClient {
    host: &'static str,
}
fn get_salt() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error gettting salt(system time)")
        .as_secs()
}
fn gen_sha1(input: String) -> String {
    let mut sh = Sha1::new();
    sh.input_str(input.as_str());
    sh.result_str()
}

impl Default for ShineMonitorClient {
    fn default() -> Self {
        ShineMonitorClient::new(BASE_URL)
    }
}

impl ShineMonitorClient {
    /// Creates a new ShineMonitorClient client with a custom host url
    pub fn new(host: &'static str) -> Self {
        ShineMonitorClient { host }
    }

    async fn get<R: DeserializeOwned>(&self, parameters: &str) -> Result<R, Error> {
        reqwest::get(format!("{}{}", self.host, parameters))
            .await?
            .json()
            .await
    }
    /// Check API server status
    pub async fn ping(&self) -> Result<SimplePing, Error> {
        self.get("/").await
    }

    fn auth_url(&self, user: &str, pass: &str) -> String {
        let (salt, action, passsha1): (u64, String, String) = (
            get_salt(),
            format!("&action=auth&usr={}&company-key={}", user, COMPANY_KEY),
            gen_sha1(pass.to_string()),
        );

        let sign = gen_sha1(format!("{}{}{}", salt, passsha1, action));
        //  println!("Sign: {}", sign);
        format!("/?sign={}&salt={}{}", sign, salt, action)
    }

    /// Auth with the api
    pub async fn auth(&self, user: &str, pass: &str) -> Result<Auth, Error> {
        //	println!("URL: {}", self.auth_url(user,pass).as_str());
        self.get(self.auth_url(user, pass).as_str()).await
    }
}

impl Auth {
    fn pid_url(&self) -> String {
        let (sal, auth): (u64, &AuthDat) = (get_salt(), &self.dat);
        let (token, secret): (String, String) = (auth.clone().token, auth.clone().secret);
        let action = "&action=queryPlantsInfo&i18n=pt_BR&lang=pt_BR";
        let sign = gen_sha1(format!("{}{}{}{}", sal, secret, token, action));
        format!(
            "{}/?sign={}&salt={}&token={}{}",
            BASE_URL, sign, sal, token, action
        )
    }

    /// Get the plantid
     pub async fn pid(&self) -> Result<Pid, Error> {
        //println!("{}",self.pid_url());
        reqwest::get(self.pid_url().as_str()).await?.json().await
    }

    async fn plant_url(&self) -> String {
        let (sal, auth): (u64, &AuthDat) = (get_salt(), &self.dat);
        let (token, secret): (String, String) = (auth.clone().token, auth.clone().secret);
        let pid = self
            .pid()
            .await
            .expect("Failed to get Pid")
            .dat
            .info
            .first()
            .expect("Unable to get first plant")
            .pid;
        let action = format!(
            "&action=queryPlantInfo&plantid={}&i18n=pt_BR&lang=pt_BR",
            pid
        );
        let sign = gen_sha1(format!("{}{}{}{}", sal, secret, token, action));
        format!(
            "{}/?sign={}&salt={}&token={}{}",
            BASE_URL, sign, sal, token, action
        )
    }

    /// Get Plant Info
    pub async fn plant(&self) -> Result<Plant, Error> {
        let plant_url = self.plant_url().await;
        let res = reqwest::get(plant_url).await?.json().await;
        res
    }
    async fn solar_url(&self) -> String {
        let (sal, auth): (u64, &AuthDat) = (get_salt(), &self.dat);
        let (token, secret): (String, String) = (auth.clone().token, auth.clone().secret);
        let pid = self
            .pid()
            .await
            .expect("faield to get pid")
            .dat
            .info
            .first()
            .expect("failed to get first plant")
            .pid;
        let action = format!("&action=queryPlantCurrentData&plantid={}&par=ENERGY_TODAY,ENERGY_MONTH,ENERGY_YEAR,ENERGY_TOTAL,ENERGY_PROCEEDS,ENERGY_CO2,CURRENT_TEMP,CURRENT_RADIANT,BATTERY_SOC,ENERGY_COAL,ENERGY_SO2",pid);
        let sign = gen_sha1(format!(
            "{}{}{}{}",
            sal,
            secret.as_str(),
            token.as_str(),
            action
        ));
        format!(
            "{}/?sign={}&salt={}&token={}{}",
            BASE_URL,
            sign.as_str(),
            sal,
            token.as_str(),
            action
        )
    }

    /// Get solar info
    pub async fn solar(&self) -> Result<Solar, Error> {
        let url = self.solar_url().await;
        reqwest::get(url)
            .await
            .expect("Could not get solar data")
            .json()
            .await
    }
}
