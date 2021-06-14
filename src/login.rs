use rocket::form::FromForm;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::request::Outcome;
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{Flash, Redirect};

use crate::config;
use crate::diesel_pgsql::Db;
use crate::schema;

use self::diesel::prelude::*;
use rocket_sync_db_pools::diesel;

use rocket_dyn_templates::Template;

#[derive(Debug)]
pub struct User(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<User, Self::Error> {
        let db = request.guard::<Db>().await.unwrap();
        let username: String = request
            .cookies()
            .get_private("username_cookie")
            .unwrap_or_else(|| Cookie::new("username_cookie", ""))
            .value()
	    .to_string();

        let sess_id: String = request
            .cookies()
            .get_private("session-id")
            .unwrap_or_else(|| Cookie::new("session-id", ""))
            .value()
	    .to_string();

	let uname = username.clone();
        let password: String = db
            .run(move |conn| {
                schema::users::table
                    .select(schema::users::password)
                    .filter(schema::users::username.eq(uname))
                    .first(conn)
            })
            .await
            .expect("Could not load from database");

        let cur_sess_id = sha256sum(username.to_string() + password.as_str());
        let outcome: Outcome<User, Self::Error>;

        if sess_id == cur_sess_id {
            outcome = Outcome::Success(User(username.to_string()));
        } else {
            outcome = Outcome::Forward(());
	}
        outcome
    }
}

#[derive(FromForm, Debug, Clone, Copy)]
struct Login<'r> {
    username: &'r str,
    password: &'r str,
}

#[derive(FromForm, Debug)]
struct Password<'r> {
    #[field(validate = len(6..20))]
    #[field(validate = eq(self.second))]
    first: &'r str,
    #[field(validate = eq(self.first))]
    second: &'r str,
}

#[derive(FromForm, Debug)]
struct Signup<'r> {
    #[field(validate = len(1..30))]
    username: &'r str,
    password: Password<'r>,
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    email: &'r str,
}

#[get("/login")]
fn login_page(
    config: config::Config,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let context = config::Context::new(config::i18n(config), "", &flash);

    println!("{:#?}", context);
    Template::render("login", context)
}

#[post("/login", data = "<login>")]
async fn post_login(
    jar: &CookieJar<'_>,
    login: Form<Login<'_>>,
    db: Db,
) -> Flash<Redirect> {
    let username = login.username.to_string();
    let password: String = db
        .run(move |conn| {
            schema::users::table
                .select(schema::users::password)
                .filter(schema::users::username.eq(username))
                .first(conn)
        })
        .await
        .expect("Could not load from database");

    let sha_login_pass = sha256sum(login.password.to_string());
    let userinfo =
        sha256sum(login.username.to_string() + sha_login_pass.as_str());

    if sha_login_pass == password {
        jar.add_private(Cookie::new(
            "username_cookie",
            login.username.to_string(),
        ));
        jar.add_private(Cookie::new("session-id", userinfo));

        Flash::success(Redirect::to(uri!(login_page)), "Successful login")
    } else {
        Flash::error(Redirect::to(uri!(login_page)), "Invalid user/pass")
    }
}

pub fn stage() -> Vec<rocket::Route> {
    routes![login_page, post_login]
}

fn sha256sum(s: String) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(s.as_str());
    hasher.result_str()
}
