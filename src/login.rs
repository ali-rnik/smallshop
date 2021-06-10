use rocket::form::FromForm;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{Flash, Redirect};

use crate::config;
use rocket_dyn_templates::Template;

#[derive(Debug)]
pub struct User(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<User, Self::Error> {
        request
            .cookies()
            .get_private("userinfo_hash")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(User)
            .or_forward(())
    }
}

#[derive(FromForm, Debug)]
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
fn post_login(
    jar: &CookieJar<'_>,
    login: Form<Login<'_>>
) -> Flash<Redirect> {
    if login.username == "Ali" && login.password == "password" {
        let mut hasher = Sha256::new();
        hasher.input_str(&(login.username.to_string() + login.password));

        jar.add_private(Cookie::new("userinfo_hash", hasher.result_str()));
        Flash::success(
            Redirect::to(uri!(login_page)),
            "Successful login",
        )
    } else {
	Flash::error(
            Redirect::to(uri!(login_page)),
            "Invalid user/pass",
        )
    }
}


pub fn stage() -> Vec<rocket::Route> {
    routes![login_page, post_login]
}
