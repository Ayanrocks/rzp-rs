extern crate reqwest;

use models::sdk_client::Client;

mod constants;
mod models;

#[cfg(test)]
mod tests {
    use crate::models::sdk_client::CreateCustomer;

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

        let customer_response = client.create_order(create_customer_payload).unwrap();

        println!("{:?}", customer_response);
    }
}

pub fn init_client(key_id: &str, key_secret: &str) -> Client {
    return Client::new_client(key_id, key_secret);
}
