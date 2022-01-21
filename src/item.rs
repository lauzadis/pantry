use serde_derive::{Serialize, Deserialize};
use bigdecimal::BigDecimal;
use mongodb::bson;
use crate::mongo::MongoState;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub name: String,
    pub quantity: Option<i32>,
    pub mass: Option<BigDecimal>,
}

#[get("/")]
pub async fn get_items(mongo: &State<MongoState>) -> Json<Vec<Item>> {
    let db = &mongo.db;
    let items: Collection<Item> = db.collection::<Item>("item");
    
    let cursor = items.find(None, None).await.unwrap();
    let results: Vec<Item> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
    println!("{:?}", results);
    Json(results)
}

#[post("/", data = "<new_item>")]
pub async fn create_item(mongo: &State<MongoState>, new_item: Json<Item>) -> Json<Item> {
    let new_item = new_item.into_inner();

    let db = &mongo.db;
    let items: Collection<Item> = db.collection::<Item>("item");
    
    let inserted_id = items.insert_one(new_item, None).await.unwrap().inserted_id;

    let filter = bson::doc! {"_id": inserted_id};
    match items.find_one(filter, None).await.unwrap() {
        None => panic!("Failed to insert item"),
        Some(item) => Json(item)
    }
}

#[put("/<item_id>", data="<updated_item>")]
pub async fn update_item(mongo: &State<MongoState>, item_id: &str, updated_item: Json<Item>) -> Json<Item> {
    let item_id = bson::oid::ObjectId::parse_str(item_id).expect("Failed to parse ObjectId");
    let updated_item = bson::to_bson(&updated_item.into_inner()).unwrap();
    let updated_item = doc!{"$set": updated_item};

    let items = mongo.db.collection("item");
    let item_to_update = items.find_one(doc!{"_id": item_id}, None).await.unwrap().expect("Failed to find item");

    items.update_one(item_to_update, updated_item, None).await.unwrap();

    let items = mongo.db.collection::<Item>("item");
    let updated_item = items.find_one(doc!{"_id": item_id}, None).await.unwrap().unwrap();
    Json(updated_item)
}

#[delete("/<item_id>")]
pub async fn delete_item(mongo: &State<MongoState>, item_id: &str) -> Json<Item> {    
    let item_id = bson::oid::ObjectId::parse_str(item_id).expect("Failed to parse ObjectId");

    let items = mongo.db.collection::<Item>("item");

    let filter = bson::doc! {"_id": item_id};
    match items.find_one_and_delete(filter, None).await {
        Ok(Some(item)) => Json(item),
        _ => panic!("Failed to delete item")
    }
}

#[get("/")]
pub async fn item_index(mongo: &State<MongoState>) -> Template {
    let mut context = HashMap::<String, Vec<Item>>::new();
    let items = get_items(mongo).await.into_inner();

    context.insert("items".to_string(), items);
    Template::render("item", &context)
}




use rocket::{serde::{json::Json}};

use rocket_dyn_templates::{Template};
use std::collections::HashMap;
use std::env;
use dotenv::dotenv;

use mongodb::{Client, Database, Collection};
use rocket::{get, State};
use futures::stream::{TryStreamExt};
use bson::{doc};

