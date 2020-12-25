use crate::schema::users;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub email_adress: String,
    pub created_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUsers<'a> {
    pub email_adress: &'a str,
    pub created_at: NaiveDateTime
}
