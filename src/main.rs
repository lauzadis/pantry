#[macro_use] extern crate rocket;

mod item;
use crate::item::{Item};

use rocket::{serde::{json::Json}};

use rocket_dyn_templates::{Template};
use std::collections::HashMap;
use std::env;
use dotenv::dotenv;

use mongodb::{Client, Database, Collection};
use rocket::{get, State};
use futures::stream::{TryStreamExt};
use bson::{doc};


#[get("/")]
async fn get_items(mongo: &State<MongoState>) -> Json<Vec<Item>> {
    let db = &mongo.db;
    let items: Collection<Item> = db.collection::<Item>("item");
    
    let cursor = items.find(None, None).await.unwrap();
    let results: Vec<Item> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
    println!("{:?}", results);
    Json(results)
}

#[post("/", data = "<new_item>")]
async fn create_item(mongo: &State<MongoState>, new_item: Json<Item>) -> Json<Item> {
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
async fn update_item(mongo: &State<MongoState>, item_id: &str, updated_item: Json<Item>) -> Json<Item> {
    let item_id = bson::oid::ObjectId::parse_str(item_id).expect("Failed to parse ObjectId");
    let updated_item = bson::to_bson(&updated_item.into_inner()).unwrap();
    let updated_item = doc!{"$set": updated_item};

    let items = mongo.db.collection("item");
    let item_to_update = items.find_one(doc!{"_id": item_id}, None).await.unwrap().expect("Failed to find item");

    let update_result = items.update_one(item_to_update, updated_item, None).await.unwrap();

    let items = mongo.db.collection::<Item>("item");
    let updated_item = items.find_one(doc!{"_id": item_id}, None).await.unwrap().unwrap();
    Json(updated_item)
}

#[delete("/<item_id>")]
async fn delete_item(mongo: &State<MongoState>, item_id: &str) -> Json<Item> {    
    let item_id = bson::oid::ObjectId::parse_str(item_id).expect("Failed to parse ObjectId");

    let items = mongo.db.collection::<Item>("item");

    let filter = bson::doc! {"_id": item_id};
    match items.find_one_and_delete(filter, None).await {
        Ok(Some(item)) => Json(item),
        _ => panic!("Failed to delete item")
    }
}

#[get("/")]
async fn item_index(mongo: &State<MongoState>) -> Template {
    let mut context = HashMap::<String, Vec<Item>>::new();
    let items = get_items(mongo).await.into_inner();

    context.insert("items".to_string(), items);
    Template::render("item", &context)
}

#[get("/")]
async fn index() -> Template {
    let context = HashMap::<String, Vec<Item>>::new();
    // context.insert("num_items".to_string(), IndexContext::from(5));
    Template::render("index", &context)
}

struct MongoState {
    db: Database,
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client = Client::with_uri_str(database_url).await.unwrap();

    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let db = client.database(&database_name);

    for coll_name in db.list_collection_names(None).await {
        println!("collection: {:?}", coll_name);
    }

    rocket::build()
    .manage(MongoState {db})
    .attach(Template::fairing())
    .mount("/", routes![index])
    .mount("/item", routes![item_index])
    .mount("/api/item", routes![get_items, create_item, delete_item, update_item])
}
