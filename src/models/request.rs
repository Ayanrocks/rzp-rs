use std::collections::HashMap;

use reqwest::{RequestBuilder, Response};

use crate::constants::{BASE_URL, SDK_NAME, SDK_VERSION};
use crate::models::card::CardResponse;
use std::borrow::Borrow;

pub struct Request {
    auth: Auth,
    headers: HashMap<String, String>,
    version: String,
    sdk_name: String,
    // AppDetails map[string]string
    base_url: String,
    http_client: reqwest::Client,
}

struct Auth {
    key_id: String,
    key_secret: String,
}

impl Request {
    pub fn init(key_id: &str, key_secret: &str) -> Request {
        let client = reqwest::Client::new();
        let c = Request {
            auth: Auth {
                key_id: key_id.to_string(),
                key_secret: key_secret.to_string(),
            },
            headers: Default::default(),
            version: SDK_VERSION.to_string(),
            sdk_name: SDK_NAME.to_string(),
            base_url: BASE_URL.to_string(),
            http_client: client,
        };

        c
    }

    pub async fn do_request(&self, req: &RequestBuilder) -> Response {

            let res = req.send().await?;
            if res.status() == reqwest::StatusCode::BAD_REQUEST {
                println!("Error Response from Razorpay: ");
                todo!("Add error handling and return error response as well as success response");
            }
            return res;

    }

    pub fn get<T>(&self, url: &str) -> T {
        let final_url = format!("{}/{}", self.base_url, url);
        println!("Sending request: {}", final_url);
        let mut req = self.http_client
            .post(final_url)
            .basic_auth(&self.auth.key_id, Some(&self.auth.key_secret));
        let response = self.do_request(&req);

        return response.json::<T>();
    }

    pub fn post<T>(&self, url: &str, body: Option<HashMap<String, String>>) -> T {
        let final_url = format!("{}/{}", self.base_url, url);
        println!("Sending request: {}", final_url);
        let req_payload = body.unwrap();
        let mut req = self.http_client
            .post(final_url)
            .basic_auth(&self.auth.key_id, Some(&self.auth.key_secret))
            .json(&req_payload);
        let response = self.do_request(&req);

        return response.json::<T>();
    }
}
