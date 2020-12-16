mod utils;

use wasm_bindgen::prelude::*;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod models;
pub mod schema;

use models::books::Book;
// use schema::books;
use self::models::posts::NewPost;
use self::models::books::NewBook;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("URL not defined");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_posts(book_id: i32, user_id: i32,
                    page: i32, text: &str) -> usize {

    use crate::schema::posts;
    let new_post = NewPost {book_id, user_id, page, text};

    let conn = establish_connection();

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&conn)
        .expect("Error saving new posts")
}

pub fn register_new_book(title: &str) -> usize {

    use crate::schema::books;
    let new_book = NewBook {title: title};

    let conn = establish_connection();

    diesel::insert_into(books::table)
        .values(&new_book)
        .execute(&conn)
        .expect("Error saving new books")
}

pub fn get_popular_books() -> String {
    use schema::books::dsl::{num_posts, books};

    let conn = establish_connection();
    let results = books
        .order(num_posts.desc())
        .limit(3)
        .load::<Book>(&conn)
        .expect("Error loading");

    let serialized = serde_json::to_string(&results).unwrap();
    serialized
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    unsafe{alert("Hello, math-net!");}
}

#[cfg(test)]
mod test {
    use std::any::type_name;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
    #[test]
    fn test_establish_connection() {
        use crate::establish_connection;

        let _ = establish_connection();
    }

    #[test]
    fn test_register_new_book() {
        use crate::register_new_book;

        let test_title = "関数解析";
        assert_eq!(1, register_new_book(test_title));
    }
}
