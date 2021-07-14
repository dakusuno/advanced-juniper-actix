table! {
    products (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        price -> Float8,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
    }
}

joinable!(products -> users (user_id));

allow_tables_to_appear_in_same_query!(
    products,
    users,
);
