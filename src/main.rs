#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate log;
extern crate env_logger;
extern crate traqbot;
use traqbot::router::handler::*;
use traqbot::database::Database;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    rocket::ignite()
        .mount("/", routes![
            index,
            empty,
            message
        ])
        .attach(Database::fairing())
        .launch();
}
