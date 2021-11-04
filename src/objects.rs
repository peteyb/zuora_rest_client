use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ZuoraSubscriptionResponse {
    pub done: bool,
    pub records: Vec<Subscription>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: u32,
}
