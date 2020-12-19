table! {
    books (book_id) {
        book_id -> Int4,
        title -> Varchar,
        num_posts -> Int4,
    }
}

table! {
    posts (post_id) {
        post_id -> Int4,
        book_id -> Int4,
        user_id -> Int4,
        page -> Int4,
        body -> Text,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        email_adress -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    posts,
    users,
);
