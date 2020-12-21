use crate::schema::books;

#[derive(Queryable)]
#[derive(Serialize)]
pub struct Book {
    pub book_id: i32,
    pub title: String,
    pub num_posts: i32,
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook<'a> {
    pub title: &'a str,
}
