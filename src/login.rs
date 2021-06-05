use std::collections::HashMap;

use rocket::form::FromForm;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, CookieJar};
use rocket::form::Form;

use rocket_dyn_templates::Template;
use crate::config;

#[derive(Debug)]
pub struct User(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>)
			  -> request::Outcome<User, Self::Error>
    {
	request.cookies()
	    .get_private("userinfo_hash")
	    .and_then(|cookie| cookie.value().parse().ok())
	    .map(User)
	    .or_forward(())
    }
}

#[derive(FromForm)]
struct Login<'r> {
    username: &'r str,
    password: &'r str
}

#[get("/login")]
fn login_page(config: config::Config,
	      flash: Option<FlashMessage<'_>>) -> Template {
    
    let mut context = HashMap::new();
    config::i18n(config, &mut context);
    let tup;
    match flash {
	Some(i) => tup = i.into_inner(),
	None => tup = ("none".to_string(), "none".to_string())
    };
    context.insert("flash_kind", tup.0);
    
    Template::render("login", context)
}

#[post("/login", data = "<login>")]
fn post_login(jar: &CookieJar<'_>, login: Form<Login<'_>>)
	      -> Result<Flash<Redirect>, Flash<Redirect>>
{
    if login.username == "Ali" && login.password == "password" {
	let mut hasher = Sha256::new();
	hasher.input_str(&(login.username.to_string() + login.password));

	jar.add_private(Cookie::new("userinfo_hash", hasher.result_str()));
        Ok(Flash::success(Redirect::to(uri!(login_page)), "Successful login"))
    } else {
	Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid user/pass"))
    }
    
}

pub fn stage() -> Vec<rocket::Route> {
    routes![login_page, post_login]
}

