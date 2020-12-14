table! {
    sensitive_regular (id) {
        id -> Int4,
        regulars -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    sensitive_words (id) {
        id -> Int4,
        words -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    sensitive_regular,
    sensitive_words,
);
