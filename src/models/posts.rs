use crate::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub post_id: i32,
    pub book_id: i32,
    pub user_id: i32,
    pub page: i32,
    pub text: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub book_id: i32,
    pub user_id: i32,
    pub page: i32,
    pub text: &'a str,
}
