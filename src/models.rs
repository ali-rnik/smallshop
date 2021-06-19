use crate::schema::{products, users, orders};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rocket::serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::signup;

#[derive(
    FromForm,
    Debug,
    Clone,
    Deserialize,
    Serialize,
    Queryable,
    Insertable,
    AsChangeset,
)]
#[serde(crate = "rocket::serde")]
#[table_name = "products"]
pub struct Product {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub name: String,
    pub number_weight: f32,
    pub is_numeric: bool,
    pub supplier_id: i32,
    pub produce_date: SystemTime,
    pub expire_date: SystemTime,
    pub address: String,
    pub unit_price: i64,
    pub benefit_percent: f32,
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
    pub role: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "orders"]
pub struct Order {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub user_id: i32,
    pub products_id: String,
    pub ship_address: String,
    pub payed_at: SystemTime,
    pub recieved_at: SystemTime,
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
