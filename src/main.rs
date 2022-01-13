#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod item;
use crate::item::{Item, ChangedItem};

use crate::schema::items;

use rocket::serde::{json::Json};
use rocket_sync_db_pools::database;
use diesel::prelude::*;

use rocket_dyn_templates::{Template};
use std::collections::HashMap;

#[database("postgres")]
pub struct DbConn(diesel::PgConnection);

#[get("/")]
async fn get_items(conn: DbConn) -> Json<Vec<Item>> {
    let results = conn.run(|c|
        items::table.load::<Item>(c)
        .expect("Error loading Items"))
        .await;

    Json(results)
}

#[post("/", data = "<new_item>")]
async fn create_item(conn: DbConn, new_item: Json<ChangedItem>) -> Json<Item> {
    let item = conn.run(|c| 
        diesel::insert_into(items::table)
        .values(new_item.into_inner())
        .get_result::<Item>(c)
        .expect("Error creating Item")
    ).await;

    Json(item)
}

#[put("/<item_id>", data="<updated_item>")]
async fn update_item(item_id: i32, conn: DbConn, updated_item: Json<ChangedItem>) -> Json<Item> {
    let /*mut*/ updated_item = updated_item.into_inner();
    // if updated_item.quantity == Some(0) {
    //     updated_item.quantity = None;
    // } else if updated_item.mass == Some(BigDecimal::from_str("0.0").unwrap()) {
    //     updated_item.mass = None;
    // }

    let item = conn.run(move |c|
        diesel::update(
            items::table.find(item_id))
            .set(updated_item)
            .get_result::<Item>(c)
            .expect("Unable to update Item")
    ).await;

    Json(item)
}

#[delete("/<item_id>")]
async fn delete_item(item_id: i32, conn: DbConn) -> Json<Item> {
    let item = conn.run(move |c|
        diesel::delete(items::table.find(item_id))
        .get_result::<Item>(c)
        .expect("Unable to delete Item")
    ).await;

    Json(item)
}

#[get("/")]
async fn index() -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert("name".to_string(), "matas".to_string());
    Template::render("index", &context)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(DbConn::fairing())
    .attach(Template::fairing())
    .mount("/", routes![index])
    .mount("/api", routes![get_items, create_item, update_item, delete_item])
}