use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub email_adress: i32
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUsers<'a> {
    pub email_adress: &'a str
}
