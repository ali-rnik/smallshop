table! {
    products (id) {
        id -> Integer,
        name -> Text,
	is_numeric -> Bool,
        number_weight -> Float,
        supplier_id -> Integer,
        produce_date -> Timestamp,
        expire_date -> Timestamp,
        address -> Text,
        unit_price -> BigInt,
	benefit_percent -> Float,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
	password -> Text,
	email -> Text,
	joined -> Timestamp,
	role -> Text,
    }

}

table! {
    orders (id) {
        id -> Nullable<Integer>,
        user_id -> i32,
	products_id -> Text,
	ship_address -> Text,
	payed_at -> Timestamp,
	recieved_at -> Timestamp,
    }

}



