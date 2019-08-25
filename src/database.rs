use rocket_contrib::databases::diesel;

pub mod models;
pub mod schema;
pub mod operation;

#[database("mysql")]
pub struct Database(diesel::MysqlConnection);
