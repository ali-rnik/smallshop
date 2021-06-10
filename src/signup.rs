use rocket::form::{Form, Contextual};
use rocket::request::FlashMessage;

use rocket_dyn_templates::Template;

use crate::config;

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
    #[field(validate = len(4..30))]
    username: &'r str,
    password: Password<'r>,
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    email: &'r str,
}

#[get("/signup")]
fn signup(
    config: config::Config,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let context = config::Context::new(config::i18n(config), "", &flash);
    Template::render("signup", context)
}

#[post("/signup", data = "<signup>")]
fn signup_submit<'r>(
    config: config::Config,
    signup: Form<Contextual<'r ,Signup<'r>>>
) -> Template {
    let context = config::Context::new(config::i18n(config), &signup.context,
				       "");

    let template = match signup.value {
	Some(ref _suval) => Template::render("index", &context),
	None => Template::render("signup", &context),

    };

    println!("{:#?}", &context);

    template
}

pub fn stage() -> Vec<rocket::Route> {
    routes![signup, signup_submit]
}

