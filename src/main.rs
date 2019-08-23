#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate traqbot;
use traqbot::router::handler::*;
use traqbot::database::schema::Database;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            empty
        ])
        .attach(Database::fairing())
        .launch();
}
