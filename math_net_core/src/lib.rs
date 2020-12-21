/*
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub struct BookId(pub i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Book {
    pub book_id: BookId,
    pub title: String,
    pub num_posts: i32
}
*/

pub mod book;
