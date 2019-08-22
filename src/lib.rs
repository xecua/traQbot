#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_imports)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate regex;

pub mod router;
