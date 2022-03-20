use std::collections::HashMap;

use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::models::request::Request;
use crate::models::api_objects::ErrorResponse;

pub struct Client {
    request_client: Request,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCustomer {
    pub name: String,
    pub email: String,
    pub contact: String,
    pub fail_existing: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerResponse {
    pub id: String,
    pub entity: String,
    pub name: String,
    pub email: String,
    pub contact: String,
    pub gstin: String,
    pub notes: Notes,
    #[serde(rename = "created_at ")]
    pub created_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notes {
    #[serde(rename = "notes_key_1")]
    pub notes_key_1: String,
    #[serde(rename = "notes_key_2")]
    pub notes_key_2: String,
}

impl Client {
    pub fn new_client(key_id: &str, key_secret: &str) -> Client {
        let c = Client {
            request_client: Request::init(key_id, key_secret),
        };

        c
    }


    pub fn create_order(&self, req_payload: CreateCustomer) -> Result<CreateCustomerResponse, ErrorResponse> {
        let mut hm = HashMap::new();
        hm.insert(String::from("name"), req_payload.name);
        hm.insert(String::from("email"), req_payload.email);
        hm.insert(String::from("contact"), req_payload.contact);
        hm.insert(String::from("fail_existing"), req_payload.fail_existing);


        self.request_client.post::<CreateCustomerResponse>("customers", Some(hm))
    }
}
