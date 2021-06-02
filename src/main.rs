#[macro_use] extern crate rocket;

#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

use std::collections::HashMap;

use fluent_templates::{FluentLoader, static_loader};

pub mod login;
pub mod store;
pub mod models;
pub mod diesel_pgsql;
pub mod schema;

use rocket_dyn_templates::Template;

static_loader! {
    static LOCALES = {
	locales: "locales",
	fallback_language: "en-US",
	// Removes unicode isolating marks around arguments, you typically
	// should only set to false when testing.
	customise: |bundle| bundle.set_use_isolating(false),
    };
}

#[get("/")]
fn index(user: login::User) -> Template {
    let mut context = HashMap::new();

    context.insert("userinfo_hash", user.0);
    context.insert("lang", "fa".to_string());
    context.insert("dir", "rtl".to_string());
    Template::render("index", context)
}

#[get("/", rank = 2)]
fn no_auth_index() -> Template {
    let mut context = HashMap::new();
    context.insert("lang", "fa".to_string());
    context.insert("dir", "rtl".to_string());
    Template::render("index", context)
}

#[launch]
fn rocket() -> _ {

    rocket::build()
	.attach(Template::custom(|engines| {
	    engines
		.handlebars
		.register_helper("fluent",
				 Box::new(FluentLoader::new(&*LOCALES)));

	}))
	.attach(diesel_pgsql::stage())
	.mount("/", routes![index, no_auth_index])
	.mount("/", login::stage())
	.mount("/store", store::stage())
}

