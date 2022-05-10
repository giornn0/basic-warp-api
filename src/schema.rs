table! {
    access_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Text,
        valid_until -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    concepts (id) {
        id -> Int4,
        name -> Varchar,
        amount -> Numeric,
        image -> Nullable<Text>,
        user_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        lastname -> Varchar,
        state -> Nullable<Bool>,
        password -> Text,
        email -> Varchar,
        balance -> Nullable<Numeric>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(access_tokens -> users (user_id));
joinable!(concepts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    concepts,
    users,
);
