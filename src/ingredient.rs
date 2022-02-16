use crate::item::Item;
use crate::measurement::Measurement;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    pub item_id: bson::oid::ObjectId,
    pub amount: Measurement,
}