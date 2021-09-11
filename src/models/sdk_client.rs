use crate::models::request::Request;

pub struct Client {
    request_client: Request,
}

impl Client {
    pub fn new_client(key_id: &str, key_secret: &str) -> Client {
        let c = Client {
            request_client: Request::init(key_id, key_secret),
        };

        c
    }


}
