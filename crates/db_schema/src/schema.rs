// @generated automatically by Diesel CLI.

diesel::table! {
    cards (id) {
        id -> Varchar,
        user_id -> Varchar,
        product_id -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Varchar,
        name -> Varchar,
        parent -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    category_aggregations (id) {
        id -> Varchar,
        category_id -> Varchar,
        products -> Int8,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Varchar,
        user_id -> Varchar,
        product_id -> Varchar,
        comment -> Nullable<Text>,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    identifies (id) {
        id -> Varchar,
        user_id -> Varchar,
        usr_name -> Varchar,
        usr_pwd -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    order_items (id) {
        id -> Varchar,
        order_id -> Varchar,
        product_id -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    payments (id) {
        id -> Varchar,
        order_id -> Varchar,
        amount -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        image -> Text,
        price -> Int8,
        quantity -> Int8,
        category_id -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_aggregations (id) {
        id -> Varchar,
        user_id -> Varchar,
        orders -> Int8,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        fname -> Varchar,
        lname -> Varchar,
        gender -> Varchar,
        email -> Varchar,
        phone_number -> Varchar,
        address -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Varchar,
        user_name -> Varchar,
        pwd -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    usr_orders (id) {
        id -> Varchar,
        user_id -> Varchar,
        order_date -> Timestamp,
        status -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(cards -> products (product_id));
diesel::joinable!(cards -> users (user_id));
diesel::joinable!(category_aggregations -> categories (category_id));
diesel::joinable!(comments -> products (product_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(identifies -> users (user_id));
diesel::joinable!(order_items -> products (product_id));
diesel::joinable!(order_items -> usr_orders (order_id));
diesel::joinable!(payments -> usr_orders (order_id));
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(user_aggregations -> users (user_id));
diesel::joinable!(usr_orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cards,
    categories,
    category_aggregations,
    comments,
    identifies,
    order_items,
    payments,
    products,
    user_aggregations,
    users,
    usr_orders,
);
