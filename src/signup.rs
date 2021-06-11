use rocket::form::{Contextual, Form};
use rocket::request::FlashMessage;

use rocket_dyn_templates::Template;

use crate::config;
use crate::diesel_pgsql::Db;
use crate::models;
use crate::schema;

use self::diesel::prelude::*;
use rocket_sync_db_pools::diesel;

#[derive(FromForm, Debug, Clone, Copy)]
pub struct Password<'r> {
    #[field(validate = len(6..20))]
    #[field(validate = eq(self.second))]
    pub first: &'r str,
    #[field(validate = eq(self.first))]
    second: &'r str,
}

#[derive(FromForm, Debug, Clone, Copy)]
pub struct Signup<'r> {
    #[field(validate = len(4..30))]
    pub username: &'r str,
    pub password: Password<'r>,
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    pub email: &'r str,
}

#[get("/signup")]
fn signup(config: config::Config, flash: Option<FlashMessage<'_>>) -> Template {
    let context = config::Context::new(config::i18n(config), "", &flash);
    Template::render("signup", context)
}

#[post("/signup", data = "<signup>")]
async fn signup_submit<'r>(
    config: config::Config,
    signup: Form<Contextual<'r, Signup<'r>>>,
    db: Db,
) -> Template {
    let mut render_page = "signup";

    if signup.value.is_some() {
        render_page = "index";
        let user = models::User::new(&signup.value.unwrap());
        db.run(move |conn| {
            diesel::insert_into(schema::users::table)
                .values(user)
                .execute(conn)
        })
        .await
        .expect("Could not insert into db!");
    }

    let context =
        config::Context::new(config::i18n(config), &signup.context, "");

    Template::render(render_page, &context)
}

pub fn stage() -> Vec<rocket::Route> {
    routes![signup, signup_submit]
}
