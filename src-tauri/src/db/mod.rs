extern crate dotenv;

pub mod models;
use crate::schema::*;
use diesel::prelude::*;
use models::{NewTodo, Todo};


pub fn establish_connection(database_url:&str) -> SqliteConnection { // creates a new connection to the DB and returns reference
    // let database_url = "./store.sqlite";
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