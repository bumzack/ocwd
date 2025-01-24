// @generated automatically by Diesel CLI.

diesel::table! {
    order_items (id) {
        id -> Int4,
        #[max_length = 1000]
        order_id -> Varchar,
        #[max_length = 1000]
        item_id -> Varchar,
        #[max_length = 1000]
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        price -> Float8,
        #[max_length = 1000]
        state -> Nullable<Varchar>,
        #[max_length = 1000]
        additional_info_1 -> Nullable<Varchar>,
        #[max_length = 1000]
        additional_info_2 -> Nullable<Varchar>,
        item_created -> Timestamptz,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        #[max_length = 1000]
        order_id -> Varchar,
        #[max_length = 1000]
        buyer_id -> Varchar,
        #[max_length = 1000]
        buyer_name -> Nullable<Varchar>,
        #[max_length = 1000]
        erp_order_number -> Nullable<Varchar>,
        #[max_length = 1000]
        state -> Nullable<Varchar>,
        #[max_length = 1000]
        additional_info_1 -> Nullable<Varchar>,
        #[max_length = 1000]
        additional_info_2 -> Nullable<Varchar>,
        number_items -> Int4,
        blacklisted -> Bool,
        order_created -> Timestamptz,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    order_items,
    orders,
);
