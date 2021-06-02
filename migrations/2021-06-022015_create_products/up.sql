-- Your SQL goes here
CREATE TABLE products (
    product_id SERIAL PRIMARY KEY,
    product_name TEXT NOT NULL,
    number_weight TEXT NOT NULL,
    supplier TEXT NOT NULL,
    produce_date TEXT NOT NULL,
    expire_date TEXT NOT NULL,
    address TEXT NOT NULL,
    unit_price TEXT NOT NULL
)
