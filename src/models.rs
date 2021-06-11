use crate::schema::{products, users};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rocket::serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::signup;

#[derive(
    FromForm, Debug, Clone, Deserialize, Serialize, Queryable, Insertable,
)]
#[serde(crate = "rocket::serde")]
#[table_name = "products"]
pub struct Product {
    #[serde(skip_deserializing)]
    pub product_id: Option<i32>,
    pub product_name: String,
    pub number_weight: String,
    pub supplier: String,
    pub produce_date: String,
    pub expire_date: String,
    pub address: String,
    pub unit_price: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub email: String,
    pub joined: SystemTime,
}

impl User {
    pub fn new(signup: &signup::Signup) -> Self {
        let mut hasher = Sha256::new();
        hasher.input_str(signup.password.first);

        Self {
            id: None,
            username: signup.username.to_string(),
            password: hasher.result_str(),
            email: signup.email.to_string(),
            joined: SystemTime::now(),
        }
    }
}
