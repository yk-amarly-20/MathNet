use crate::schema::posts;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Post {
    pub post_id: i32,
    pub book_id: i32,
    pub user_id: i32,
    pub page: i32,
    pub body: String,
    pub posted_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub book_id: i32,
    pub user_id: i32,
    pub page: i32,
    pub body: &'a str,
    pub posted_at: NaiveDateTime,
}
