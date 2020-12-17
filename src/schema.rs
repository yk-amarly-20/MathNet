table! {
    books (book_id) {
        book_id -> Integer,
        title -> Text,
        num_posts -> Integer,
    }
}

table! {
    posts (post_id) {
        post_id -> Integer,
        book_id -> Integer,
        user_id -> Integer,
        page -> Integer,
        body -> Nullable<Text>,
    }
}

table! {
    users (user_id) {
        user_id -> Integer,
        email_adress -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    posts,
    users,
);
