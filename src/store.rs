use std::collections::HashMap;

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};

use rocket_dyn_templates::Template;

use crate::config;
use crate::diesel_pgsql::Db;
use crate::login;
use crate::models;
use crate::schema;

use self::diesel::prelude::*;
use rocket_sync_db_pools::diesel;

#[get("/")]
fn store(config: config::Config, user: login::User) -> Template {
    let mut data = HashMap::new();
    data.insert("session-id", user.0);
    let context = config::Context::new(config::i18n(config), data, "");

    Template::render("store", context)
}

#[get("/add_item")]
fn store_add_item(
    config: config::Config,
    _user: login::User,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let context = config::Context::new(config::i18n(config), "", flash);

    Template::render("store_add-item", context)
}

#[post("/add_item_submit", data = "<form_data>")]
async fn store_add_item_submit(
    _user: login::User,
    form_data: Form<models::Product>,
    db: Db,
) -> Flash<Redirect> {
    if form_data.product_name.len() >= 3 {
        db.run(move |conn| {
            diesel::insert_into(schema::products::table)
                .values(&*form_data)
                .execute(conn)
        })
        .await
        .expect("Could not insert into db!");

        Flash::success(
            Redirect::to(uri!("/store", store_add_item)),
            "Added Item",
        )
    } else {
        Flash::error(
            Redirect::to(uri!("/store", store_add_item)),
            "Wrong field values!",
        )
    }
}

#[get("/list_items")]
async fn store_list_items(
    config: config::Config,
    _user: login::User,
    db: Db,
) -> Template {
    let table: Vec<(Option<i32>, String, String)> = db
        .run(move |conn| {
            schema::products::table
                .select((
                    schema::products::product_id,
                    schema::products::product_name,
                    schema::products::unit_price,
                ))
                .load(conn)
        })
        .await
        .expect("Could not load id's from database");

    let context = config::Context::new(config::i18n(config), table, "");

    Template::render("store_list-items", context)
}

#[get("/list_items", rank = 2)]
async fn public_store_list_items(config: config::Config, db: Db) -> Template {
    let table: Vec<(Option<i32>, String, String)> = db
        .run(move |conn| {
            schema::products::table
                .select((
                    schema::products::product_id,
                    schema::products::product_name,
                    schema::products::unit_price,
                ))
                .load(conn)
        })
        .await
        .expect("Could not load id's from database");

    let context = config::Context::new(config::i18n(config), table, "");

    Template::render("public_store_list-items", context)
}

#[get("/delete_item/<id>")]
async fn store_delete_item(
    _user: login::User,
    db: Db,
    id: i32,
) -> Flash<Redirect> {
    db.run(move |conn| {
        diesel::delete(schema::products::table)
            .filter(schema::products::product_id.eq(id))
            .execute(conn)
    })
    .await
    .expect("Could not delete item");

    Flash::success(
        Redirect::to(uri!("/store", store_list_items)),
        "Item Deleted",
    )
}

#[get("/add_to_basket/<product_id>/<number>")]
async fn add_to_basket(
    product_id: i32,
    jar: &CookieJar<'_>,
    number: i32,
) -> Flash<Redirect> {
    jar.add_private(Cookie::new(
        "product_in_basket___".to_string() + &product_id.to_string(),
        number.to_string(),
    ));

    Flash::success(
        Redirect::to(uri!("/store", store_list_items)),
        "product added to basket",
    )
}

#[get("/show_basket")]
async fn show_basket(
    jar: &CookieJar<'_>,
    config: config::Config,
    db: Db,
) -> Template {
    let mut table: Vec<(Option<i32>, String, String)> = Vec::new();
    for c in jar.iter() {
        let id = match c.name().strip_prefix("product_in_basket___") {
            Some(id) => id,
            None => "",
        };

        if id != "" {
            let idint = id.parse::<i32>().unwrap();
            let rec: (Option<i32>, String, String) = db
                .run(move |conn| {
                    schema::products::table
                        .select((
                            schema::products::product_id,
                            schema::products::product_name,
                            schema::products::unit_price,
                        ))
                        .filter(schema::products::product_id.eq(idint))
                        .first(conn)
                })
                .await
                .unwrap_or_else(|_| (Some(0), "".to_string(), "".to_string()));

            table.push(rec);
        }
    }

    let context = config::Context::new(config::i18n(config), table, "");

    Template::render("store_show-basket", context)
}

/*#[get("/buy_and_pay")]*/

pub fn stage() -> Vec<rocket::Route> {
    routes![
        store,
        store_add_item,
        store_add_item_submit,
        store_list_items,
        public_store_list_items,
        store_delete_item,
        add_to_basket,
        show_basket
    ]
}
