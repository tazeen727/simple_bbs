table! {
    posts (thread_id, seq_number) {
        thread_id -> Int4,
        seq_number -> Int4,
        poster_name -> Varchar,
        post_body -> Varchar,
        posted_at -> Timestamptz,
        is_deleted -> Bool,
    }
}

table! {
    threads (thread_id) {
        thread_id -> Int4,
        thread_title -> Varchar,
        post_count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(posts -> threads (thread_id));

allow_tables_to_appear_in_same_query!(
    posts,
    threads,
);
