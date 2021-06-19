use rocket::http::{Cookie, CookieJar};
use rocket::outcome::Outcome::Success;
use rocket::request::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::Redirect;
use serde::Serialize;

#[derive(Debug)]
pub struct Config(pub String);

#[derive(Serialize, Debug)]
pub struct Context<T: Serialize, F: Serialize> {
    lang: String,
    dir: String,
    parent: &'static str,
    pub data: T,
    pub flash: F,
}

#[derive(Debug)]
pub struct Lang {
    pub lang: String,
    pub dir: String,
}

impl<T: Serialize, F: Serialize> Context<T, F> {
    pub fn new(
        lang_struct: Lang,
        data: T,
        flash: F,
    ) -> Self {
        Self {
            lang: lang_struct.lang,
            dir: lang_struct.dir,
	    parent: "layout",
            data,
            flash,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Config {
    type Error = std::convert::Infallible;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<Config, Self::Error> {
        let default_cookie = Cookie::new("lang", "fa");
        let cookie = request
            .cookies()
            .get_private("lang")
            .unwrap_or(default_cookie);

        let mut lang = cookie.value();
        if lang != "fa" {
            lang = "en";
        }

        let outcome: Outcome<Config, Self::Error> =
            Success(Config(lang.to_string()));

        outcome
    }
}

#[get("/config_set_lang/<lang>")]
fn set_lang(jar: &CookieJar<'_>, lang: String) -> Redirect {
    if lang == "fa" {
        println!("fa matched");
        jar.add_private(Cookie::new("lang", "fa"));
    } else {
        println!("en matched");
        jar.add_private(Cookie::new("lang", "en"));
    }

    Redirect::to(uri!("/"))
}

pub fn i18n(config: Config) -> Lang {
    if config.0 == "fa" {
        Lang {
            lang: "fa".to_string(),
            dir: "rtl".to_string(),
        }
    } else {
        Lang {
            lang: "en".to_string(),
            dir: "ltr".to_string(),
        }
    }
}

pub fn stage() -> Vec<rocket::Route> {
    routes![set_lang]
}
