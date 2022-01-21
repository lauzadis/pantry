use serde_derive::{Serialize, Deserialize};
use crate::Item;

use rocket_dyn_templates::{Template};
use std::collections::HashMap;

use mongodb::{Client, Database, Collection};
use rocket::{get, State};
use futures::stream::{TryStreamExt};
use bson::{doc};
use crate::mongo::MongoState;
use crate::ingredient::Ingredient;
use crate::measurement::{Measurement, VolumeUnit, MassUnit};


#[derive(Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<bson::oid::ObjectId>,
    name: String,
    steps: Vec<Step>,
}

#[derive(Serialize, Deserialize)]
pub struct Step {
    instructions: String,
    ingredients: Vec<Ingredient>,
}

#[get("/")]
pub async fn recipe_index(mongo: &State<MongoState>) -> Template {

    let test_recipe = Recipe {
        id: None,
        name: "Brown Basmati Rice".to_string(),
        steps: vec! [
            Step {
                instructions: "Obtain 1 cup of basmati rice.".to_string(),
                ingredients: vec! [
                    Ingredient {
                        item: Item {id: None, name: "Brown Basmati Rice".to_string(), quantity: Some(5), mass: None},
                        amount: Measurement::Volume{value: 1.0, unit: VolumeUnit::Cup},
                    }
                ]
            }
        ]
    };

    let recipes: Vec<Recipe> = vec![test_recipe];

    let mut context = HashMap::<String, Vec<Recipe>>::new();

    context.insert("recipes".to_string(), recipes);
    Template::render("recipe", &context)
}

