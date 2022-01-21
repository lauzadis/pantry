#[macro_use] extern crate rocket;

mod mongo;
use mongo::MongoState;

mod item;
use crate::item::{Item, get_items, update_item, create_item, delete_item, item_index};

mod recipe;
mod ingredient;
mod measurement;
use crate::recipe::{recipe_index};


use rocket_dyn_templates::{Template};
use std::collections::HashMap;
use std::env;
use dotenv::dotenv;

use mongodb::Client;
use rocket::get;
use bson::{doc};

#[get("/")]
async fn index() -> Template {
    let context = HashMap::<String, Vec<Item>>::new();
    // context.insert("num_items".to_string(), IndexContext::from(5));
    Template::render("index", &context)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client = Client::with_uri_str(database_url).await.unwrap();

    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let db = client.database(&database_name);

    rocket::build()
    .manage(MongoState {db})
    .attach(Template::fairing())
    .mount("/", routes![index])
    .mount("/item", routes![item_index])
    .mount("/api/item", routes![get_items, create_item, delete_item, update_item])
    .mount("/recipe", routes![recipe_index])
}
