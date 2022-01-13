use serde_derive::{Serialize, Deserialize};
use bigdecimal::BigDecimal;
use crate::schema::items;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Item {
    id: i32,
    name: String,
    quantity: Option<i32>,
    mass: Option<BigDecimal>
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")] // treat "None" as NULL instead of skipping it in the diesel::update()
#[table_name="items"]
pub struct ChangedItem {
    name: String,
    pub quantity: Option<i32>,
    pub mass: Option<BigDecimal>,
}