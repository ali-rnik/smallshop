#[macro_use] extern crate rocket;

#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

use std::collections::HashMap;

use fluent_templates::{FluentLoader, static_loader};

use rocket_dyn_templates::Template;

pub mod login;
pub mod store;
pub mod models;
pub mod diesel_pgsql;
pub mod schema;
pub mod config;

static_loader! {
    static LOCALES = {
	locales: "locales",
	fallback_language: "en-US",
	customise: |bundle| bundle.set_use_isolating(true),
    };
}

#[get("/")]
fn index(config: config::Config, user: login::User) -> Template {
    let mut context = HashMap::new();

    config::i18n(config, &mut context);
    context.insert("userinfo_hash", user.0);
    
    Template::render("index", context)
}

#[get("/", rank = 2)]
fn no_auth_index(config: config::Config) -> Template {
    let mut context = HashMap::new();
    config::i18n(config, &mut context);
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
	.mount("/config", config::stage())
}

