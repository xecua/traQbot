pub mod models;
pub mod schema;
pub mod operation;

use rocket_contrib::databases::diesel;
use diesel::prelude::*;

#[database("mysql")]
pub struct Database(diesel::MysqlConnection);
