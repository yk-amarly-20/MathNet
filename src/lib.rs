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
use self::models::posts::NewPost;
use self::models::books::NewBook;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

/// Establish connection to postgresql's database.
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    /// Write database url in .env
    let database_url = env::var("DATABASE_URL").expect("URL not defined");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/// Write new posts in database.
/// When new posts are generated, db books.num_posts have to incremented.
///
/// # Auguments
/// * `book_id` - The book id of new posts.
/// * `user_id` - The user_id of poster. (TODO: implement login)
/// * `page` - New posts' page of the book.
/// * `body` - Posted body.
///
/// # Returns
/// Num of new posts.
pub fn create_posts(book_id: i32, user_id: i32,
                    page: i32, body: &str) -> usize {

    use crate::schema::posts;
    let new_post = NewPost {book_id: book_id, user_id: user_id, page: page, body: body};

    let conn = establish_connection();

    let book: Book = schema::books::dsl::books
        .find(book_id)
        .first(&conn)
        .unwrap();
    let num_posts = book.num_posts;

    // too complex impl...
    // I want to implement more easily.
    let conn = establish_connection();
    let target = schema::books::dsl::books.filter(schema::books::dsl::book_id.eq(book_id));
    diesel::update(target)
        .set(schema::books::dsl::num_posts.eq(num_posts + 1))
        .execute(&conn)
        .expect("No books");

    let conn = establish_connection();
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&conn)
        .expect("Error saving new posts")
}

/// Register new books to database.
/// This function is called when new posts' book is not in database.
pub fn register_new_book(title: &str) -> usize {

    use crate::schema::books;
    let new_book = NewBook {title: title};

    let conn = establish_connection();

    diesel::insert_into(books::table)
        .values(&new_book)
        .execute(&conn)
        .expect("Error saving new books")
}

/// Get most popular books (three).
/// 'Popular' is defined by 'num_posts'
/// Returns information of books by Json
#[wasm_bindgen]
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

#[cfg(test)]
mod test {
    use std::any::type_name;

    #[allow(dead_code)]
    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn test_establish_connection() {
        use crate::establish_connection;

        let _ = establish_connection();
    }

    // deadlock起こるから結合テストで
    /*
    #[test]
    fn test_register_new_book_and_crate_posts() {
        use crate::register_new_book;
        use crate::create_posts;

        let test_title = "関数解析";
        let test_book_id = 2;
        let test_user_id = 0;
        let test_page = 200;
        let test_text = "ここがわかりません。";
        assert_eq!(1, register_new_book(test_title));
        assert_eq!(create_posts(test_book_id, test_user_id, test_page, test_text), 1);
    }
    */

    /*
    fn test_get_popular_books() {
        use crate::establish_connection;
        use crate::register_new_book;
        use crate::create_posts;
        use crate::get_popular_books;

        let
    }
    */
}
