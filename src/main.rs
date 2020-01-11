#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

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
            get_oauth,
            get_oauth_callback,
            ping,
            join_left,
            message
        ])
        .mount("/static", rocket_contrib::serve::StaticFiles::from("static"))
        .attach(Database::fairing())
        .launch();
}
