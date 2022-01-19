use serde_derive::{Serialize, Deserialize};
use bigdecimal::BigDecimal;
use crate::schema::items;
use crate::Item;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Recipe {
    id: i32,
    name: String,
    steps: Vec<Step>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Step {
    id: i32,
    recipe_id: i32,
    seq_id: i32,
    text: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name="recipes"]
pub struct ChangedRecipe {
    name: String,
    steps: Vec<Step>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name="steps"]
pub struct ChangedStep {
    recipe_id: i32,
    text: String,
    items: Vec<Item>,
}