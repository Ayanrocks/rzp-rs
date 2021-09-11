use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CardResponse {
    pub id: String,
    pub entity: String,
    pub name: String,
    pub last4: String,
    pub network: String,
    #[serde(rename = "type")]
    pub card_type: String,
    pub issuer: Option<String>,
    pub international: bool,
    pub emi: bool,
    pub sub_type: Option<String>,
}

// impl Card {
//     pub fn fetch(card_id: &str) -> CardResponse {
//
//     }
// }