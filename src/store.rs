use std::collections::HashMap;

use rocket::request::FlashMessage;
use rocket::response::{Redirect, Flash};
use rocket::form::Form;

use rocket_dyn_templates::Template;

use crate::login;
use crate::models;
use crate::schema;
use crate::diesel_pgsql::Db;

use rocket_sync_db_pools::diesel;
use self::diesel::prelude::*;

#[get("/")]
fn store(user: login::User) -> Template {
    let mut context = HashMap::new();
    context.insert("user_hash", user.0);
    Template::render("store", context)
}

#[get("/add_item")]
fn store_add_item(_user: login::User, flash: Option<FlashMessage<'_>>)
		  -> Template
{
    let mut context = HashMap::new();
    context.insert("flash", &flash);

    Template::render("store_add-item", context)
}

#[post("/add_item_submit", data = "<form_data>")]
async fn store_add_item_submit(_user: login::User,
			       form_data: Form<models::Product>,
			       db: Db)
    -> Flash<Redirect>
{
    if form_data.product_name == "Egg" {
	db.run(move |conn| {
	    diesel::insert_into(schema::products::table)
		.values(&*form_data)
		.execute(conn)
	}).await.expect("Could not insert into db!");
	
	Flash::success(Redirect::to(uri!("/store", store_add_item)),
		       "Added Item")
    } else {
	Flash::error(Redirect::to(uri!("/store", store_add_item)),
		     "Wrong field values!")
    }
}

#[get("/list_items")]
async fn store_list_items(_user: login::User,
			  db: Db)
    -> Template
{
    let table: Vec<(Option<i32>, String, String)> = db.run(move |conn| {
	schema::products::table
	    .select((schema::products::product_id,
		     schema::products::product_name,
		     schema::products::unit_price))
	    .load(conn)
    }).await.expect("Could not load id's from database");

    
    let mut context = HashMap::new();
    context.insert("table", table);

    Template::render("store_list-items", context)
}

#[get("/delete_item/<id>")]
async fn store_delete_item(_user: login::User,
			   db: Db, id: i32)
    -> Flash<Redirect>
{
    db.run(move |conn| {
	diesel::delete(schema::products::table)
	    .filter(schema::products::product_id.eq(id))
	    .execute(conn)
    }).await.expect("Could not delete item");


    Flash::success(Redirect::to(uri!("/store", store_list_items)),
		   "Item Deleted")
          
}

    
pub fn stage() -> Vec<rocket::Route> {
    routes![store, store_add_item, store_add_item_submit, store_list_items,
	    store_delete_item]
}

