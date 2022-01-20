use serde_derive::{Serialize, Deserialize};
use bigdecimal::BigDecimal;
use mongodb::bson;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<bson::oid::ObjectId>,
    name: String,
    quantity: Option<i32>,
    mass: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct ChangedItem {
    name: String,
    pub quantity: Option<i32>,
    pub mass: Option<BigDecimal>,
}