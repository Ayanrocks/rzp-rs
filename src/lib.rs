extern crate reqwest;

use models::sdk_client::Client;

mod constants;
mod models;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn it_create_new_client() {}
}

pub fn init_client(key_id: &str, key_secret: &str) -> Client {
    return Client::new_client(key_id, key_secret);
}
