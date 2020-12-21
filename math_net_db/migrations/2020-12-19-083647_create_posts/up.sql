CREATE TABLE books
(
    book_id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    num_posts INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE posts
(
    post_id SERIAL PRIMARY KEY,
    book_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    page INTEGER NOT NULL,
    body TEXT NOT NULL
);

CREATE TABLE users
(
    user_id SERIAL PRIMARY KEY,
    email_adress VARCHAR NOT NULL
);
