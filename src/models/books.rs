use crate::schema::books;

#[derive(Queryable)]
pub struct Book {
    pub book_id: i32,
    pub title: String
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook<'a> {
    pub title: &'a str,
}
