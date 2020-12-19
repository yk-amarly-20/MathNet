extern crate math_net;
use math_net::get_popular_books;
fn main() {
    let popular_json: String = get_popular_books();
    println!("{}", popular_json);
}
