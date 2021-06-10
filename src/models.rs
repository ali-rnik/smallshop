use rocket::serde::{Deserialize, Serialize};

use crate::schema::products;

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
