use std::time::SystemTime;
use std::time::Duration;

use rocket::request::FromRequest;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use rocket::outcome::Outcome::Success;
use rocket::request::Outcome;

use rocket::request::{self, Request};
use rocket::http::Cookie;

#[derive(Debug)]
pub struct Session(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>)
			  -> request::Outcome<Session, Self::Error>
    {
	let cookie = request
	    .cookies()
	    .get_private("custom_session")
	    .unwrap_or(Cookie::new("none", "none"));

	let mut cookie_value = cookie.value().to_string();
	if cookie_value == "none" {
	    let ip_addr = request
		.client_ip()
		.unwrap_or("0.0.0.0".parse().unwrap());
	    let sys_time = SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.unwrap_or(Duration::from_secs(1))
		.as_millis();
	    let mut hasher = Sha256::new();
	    hasher.input_str(&(ip_addr.to_string()
			       + sys_time.to_string().as_str()));
	    cookie_value = hasher.result_str();
	}

	let outcome: Outcome<Session, Self::Error> =
	    Success(Session(cookie_value));

	outcome
    }
}

