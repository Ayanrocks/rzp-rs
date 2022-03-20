extern crate reqwest;

use models::sdk_client::Client;

mod constants;
mod models;

#[cfg(test)]
mod tests {
    use crate::models::sdk_client::CreateCustomer;
    use serde::de::Unexpected::Option;
    use std::env::var;

    use super::*;

    #[test]
    fn it_create_new_client() {
        println!("Testing client creation");
        let client = init_client("", "");
        let create_customer_payload = CreateCustomer {
            name: "test-customer".to_string(),
            email: "test-customer@sezzle.in".to_string(),
            contact: "+912000020000".to_string(),
            fail_existing: "0".to_string(),
        };

        println!("Testing customer creation");

        let customer_response = client.create_order(create_customer_payload);

        let customer_response = match customer_response {
            Ok(s) => {
                assert_eq!(s.name, "test-customer".to_string(), "success response")
            }
            Err(err) => {
                assert_eq!(
                    err.error.code,
                    Some("BAD_REQUEST_ERROR".to_string()),
                    "bad request error found"
                );

                assert_eq!(
                    err.error.description,
                    Some("The api key provided is invalid".to_string()),
                    "bad request error found"
                );
            }
        };

    }
}

pub fn init_client(key_id: &str, key_secret: &str) -> Client {
    return Client::new_client(key_id, key_secret);
}
