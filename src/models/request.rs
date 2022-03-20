use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Pointer};
use std::io::Read;

use reqwest::blocking::{Client, ClientBuilder, RequestBuilder};

use crate::constants::{BASE_URL, SDK_NAME, SDK_VERSION};
use crate::models::api_objects::ErrorResponse;

pub struct Request {
    auth: Auth,
    pub headers: HashMap<String, String>,
    version: String,
    sdk_name: String,
    // AppDetails map[string]string
    pub(crate) base_url: String,
    http_client: Client,
}

struct Auth {
    key_id: String,
    key_secret: String,
}

impl Request {
    pub fn init(key_id: &str, key_secret: &str) -> Request {
        let mut clientBuilder = ClientBuilder::new();
        clientBuilder = clientBuilder.connection_verbose(true);

        let client = clientBuilder.build().expect("client");
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

    // pub fn do_request(&self, req: &RequestBuilder) -> reqwest::Result<Response> {
    //     todo!("Implement this later");
    // }

    pub fn get<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, ErrorResponse> {
        let final_url = format!("{}/{}", self.base_url, url);
        println!("Sending request: {}", final_url);
        let req = self
            .http_client
            .get(final_url)
            .basic_auth(&self.auth.key_id, Some(&self.auth.key_secret));
        let response = req.send().unwrap();

        if response.status() == reqwest::StatusCode::BAD_REQUEST {
            println!("Error Response from Razorpay: ");
            // todo!("Add error handling and return error response as well as success response");
            let err_response = response
                .json::<ErrorResponse>()
                .expect("Error For Bad Request");

            return Err(err_response);
        }

        // println!("Response Payload: {:?}", response.text());
        let success_response = response.json::<T>().expect("Response::Json()");

        return Ok(success_response);
    }

    pub fn post<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        body: Option<HashMap<String, String>>,
    ) -> Result<T, ErrorResponse> {
        let final_url = format!("{}/{}", self.base_url, url);
        println!("Sending request: {}", final_url);

        let req_payload = body.expect("body::unwrap");
        let mut req = self
            .http_client
            .post(final_url)
            .basic_auth(&self.auth.key_id, Some(&self.auth.key_secret))
            .json(&req_payload);

        let mut response = req.send().unwrap();
        println!("Response status: {:?}", response.status());

        // {
        //     // Printing response payload
        //     let mut buf: Vec<u8> = vec![];
        //     response.copy_to(&mut buf).expect("response.copy_to");
        //     println!(
        //         "Response Payload: {:?}",
        //         std::str::from_utf8(buf.as_slice())
        //     );
        // }

        if response.status() == reqwest::StatusCode::BAD_REQUEST
            || response.status() == reqwest::StatusCode::UNAUTHORIZED
        {
            println!("Error Response from Razorpay: ");
            // todo!("Add error handling and return error response as well as success response");
            let err_response = response
                .json::<ErrorResponse>()
                .expect("Error For Bad Request");

            return Err(err_response);
        }

        let success_response = response.json::<T>().expect("Response::Json()");

        return Ok(success_response);
    }
}
