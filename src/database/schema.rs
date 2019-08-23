use rocket_contrib::databases::diesel;

#[database("my_db")]
pub struct Database(diesel::MysqlConnection);