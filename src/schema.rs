table! {
    products (product_id) {
        product_id -> Nullable<Integer>,
        product_name -> Text,
        number_weight -> Text,
        supplier -> Text,
        produce_date -> Text,
        expire_date -> Text,
        address -> Text,
        unit_price -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    products,
    users,
);
