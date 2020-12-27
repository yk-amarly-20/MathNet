#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use serde::Serialize;

use rocket_contrib::templates::Template;

#[get("/hello/<name>")]
fn index(name: String) -> Template {
    #[derive(Serialize)]
    struct Context {
        name: String
    }

    let context = Context {name: name};

    Template::render("base", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
