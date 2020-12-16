-- Your SQL goes here
CREATE TABLE books
(
    book_id INTEGER NOT NULL PRIMARY KEY,
    title VARCHAR NOT NULL,
    num_posts INTEGER DEFAULT 0 NOT NULL
);

CREATE TABLE posts
(
    post_id INTEGER NOT NULL PRIMARY KEY,
    book_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    page INTEGER NOT NULL,
    "text" TEXT
);

CREATE TABLE users
(
    user_id INTEGER NOT NULL PRIMARY KEY,
    email_adress TEXT NOT NULL
);
