use crate::schema::books;
use chrono::NaiveDateTime;
use std::convert::TryInto;

use math_net_core::book;

#[derive(Queryable)]
pub struct Book {
    pub book_id: i32,
    pub title: String,
    pub num_posts: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub created_at: NaiveDateTime,
}

impl TryInto<book::Book> for Book {
    type Error = Error;

    fn try_into(self) -> Result<book::Book, Error> {
        let book = book::Book {
            book_id: book::BookId(self.book_id),
            title: self.title,
            num_posts: self.num_posts
        };

        Ok(book)
    }
}

#[derive(Debug)]
pub enum Error {
    CannotDeserializeUserSettings(String),
}
