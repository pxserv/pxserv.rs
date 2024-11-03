use reqwest::blocking::Client;
use serde_json::{json, Value};

pub struct PxServ {
    apikey: String,
}

pub struct PxServStatus {
    pub status: String,
    pub message: String,
    pub data: Option<String>,
}

impl PxServ {
    pub fn new(apikey: String) -> PxServ {
        PxServ { apikey }
    }
}
