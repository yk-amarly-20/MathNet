mod utils;

use wasm_bindgen::prelude::*;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::posts::NewPost;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_SQL").expect("URL not defined");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_posts(conn: &SqliteConnection, book_id: i32, user_id: i32,
                    page: i32, text: &str) -> usize {

    use crate::schema::posts;
    let new_post = NewPost {book_id, user_id, page, text};

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new posts")
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
