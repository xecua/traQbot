#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate log;
extern crate env_logger;
extern crate traqbot;
use traqbot::router::handler::*;
use traqbot::database::Database;

fn main() {
    env_logger::init();
    
    rocket::ignite()
        .mount("/", routes![
            index,
            ping,
            join_left,
            message
        ])
        .attach(Database::fairing())
        .launch();
}
