use rocket_contrib::databases::diesel;

#[database("mysql")]
pub struct Database(diesel::MysqlConnection);