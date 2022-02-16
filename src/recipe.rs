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
use rocket::serde::json::Json;

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
pub async fn get_recipes(mongo: &State<MongoState>) -> Json<Vec<Recipe>> {
    let db = &mongo.db;
    let recipes: Collection<Recipe> = db.collection::<Recipe>("recipe");
    
    let cursor = recipes.find(None, None).await.unwrap();
    let results: Vec<Recipe> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
    Json(results)
}

#[get("/")]
pub async fn recipe_index(mongo: &State<MongoState>) -> Template {

    // let test_recipe = Recipe {
    //     id: None,
    //     name: "Brown Basmati Rice".to_string(),
    //     steps: vec! [
    //         Step {
    //             instructions: "Obtain 1 cup of basmati rice.".to_string(),
    //             ingredients: vec! [
    //                 Ingredient {
    //                     item_id: bson::oid::ObjectId::parse_str("").expect("Failed to parse item id"),
    //                     amount: Measurement::Volume{value: 1.0, unit: VolumeUnit::Cup},
    //                 }
    //             ]
    //         }
    //     ]
    // };

    let recipes: Vec<Recipe> = get_recipes(mongo).await.into_inner();

    let mut context = HashMap::<String, Vec<Recipe>>::new();

    context.insert("recipes".to_string(), recipes);
    Template::render("recipe", &context)
}

