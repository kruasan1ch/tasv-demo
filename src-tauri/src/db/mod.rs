extern crate dotenv;

pub mod models;
use crate::schema::*;
use diesel::prelude::*;
use dotenv::dotenv;
use models::{NewTodo, Todo};
use std::env;

pub fn establish_connection() -> SqliteConnection { // creates a new connection to the DB and returns reference
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn todos_list(conn: &SqliteConnection) -> String {
    let all_todos = todos::dsl::todos
        .load::<Todo>(conn)
        .expect("Expect loading posts");
    let serialized = serde_json::to_string(&all_todos).unwrap();
    serialized
}