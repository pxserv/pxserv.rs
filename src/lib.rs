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
    pub fn new(apikey: &str) -> PxServ {
        PxServ {
            apikey: apikey.to_string(),
        }
    }

    pub fn setdata(&self, key: &str, value: &str) -> PxServStatus {
        let client = Client::new();

        let request_body = json!({
            "key":key,
            "value":value
        });

        let request = client
            .post("https://api.pxserv.net/database/setData")
            .header("apikey", &self.apikey)
            .json(&request_body)
            .send();

        match request {
            Ok(response) => {
                let response_text = response.text().unwrap();
                let json_response: Value = serde_json::from_str(&response_text).unwrap();

                let status = json_response
                    .get("status")
                    .and_then(|s| s.as_i64())
                    .expect("PxServ API ERROR")
                    .to_string();
                let message = json_response
                    .get("message")
                    .and_then(|s| s.as_str())
                    .expect("PxServ API ERROR")
                    .to_string();

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

    pub fn getdata(&self, key: &str) -> PxServStatus {
        let client = Client::new();

        let request_body = json!({
            "key":key,
        });

        let request = client
            .post("https://api.pxserv.net/database/getData")
            .header("apikey", &self.apikey)
            .json(&request_body)
            .send();

        match request {
            Ok(response) => {
                let response_text = response.text().unwrap();
                let json_response: Value = serde_json::from_str(&response_text).unwrap();

                let status = json_response
                    .get("status")
                    .and_then(|s| s.as_i64())
                    .expect("PxServ API ERROR")
                    .to_string();
                let message = json_response
                    .get("message")
                    .and_then(|s| s.as_str())
                    .expect("PxServ API ERROR")
                    .to_string();

                let data = if status == "200" {
                    Some(
                        json_response
                            .get("data")
                            .and_then(|f| f.get("value"))
                            .and_then(|s| s.as_str())
                            .expect("PxServ API ERROR")
                            .to_string(),
                    )
                } else {
                    None
                };

                return PxServStatus {
                    status,
                    message,
                    data,
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

    pub fn removedata(&self, key: &str) -> PxServStatus {
        let client = Client::new();

        let request_body = json!({
            "key":key,
        });

        let request = client
            .post("https://api.pxserv.net/database/removeData")
            .header("apikey", &self.apikey)
            .json(&request_body)
            .send();

        match request {
            Ok(response) => {
                let response_text = response.text().unwrap();
                let json_response: Value = serde_json::from_str(&response_text).unwrap();

                let status = json_response
                    .get("status")
                    .and_then(|s| s.as_i64())
                    .expect("PxServ API ERROR")
                    .to_string();
                let message = json_response
                    .get("message")
                    .and_then(|s| s.as_str())
                    .expect("PxServ API ERROR")
                    .to_string();

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
