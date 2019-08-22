#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate traqbot;
use traqbot::router::handler::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            ping_event,
            joined_left_event,
        ]).launch();
}
