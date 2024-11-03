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
    pub fn setdata(&self, key: String, value: String) -> PxServStatus {
        let client = Client::new();

        let request_body = json!({
            "apiKey":self.apikey,
            "key":key,
            "value":value
        });

        let request = client
            .post("https://api.pxserv.net/database/setData")
            .json(&request_body)
            .send();

        match request {
            Ok(response) => {
                let response_text = response.text().unwrap();
                let json_response: Value = serde_json::from_str(&response_text).unwrap();

                let status = json_response.get("status").unwrap().to_string();
                let message = json_response.get("message").unwrap().to_string();

                return PxServStatus {
                    status,
                    message,
                    data: None,
                };
            }
            Err(err) => {
                return PxServStatus {
                    status: "-1".to_string(),
                    message: err.to_string(),
                    data: None,
                }
            }
        }
    }
}
