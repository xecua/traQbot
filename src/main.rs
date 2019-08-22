#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate traqbot;
use traqbot::route::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index
        ]).launch();
}
