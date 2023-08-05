// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::App;
// use schema::todos;
use std::error::Error;
use std::{error, string, sync::Mutex};

// Start of DB example
// use super::db::{};

#[macro_use]
extern crate diesel;


use diesel::prelude::*;
pub mod schema;
pub mod db;

struct AppState {
  conn: Mutex<SqliteConnection>,
}

#[tauri::command]
fn todos_list(state: tauri::State<AppState>) -> String{
    let con = state.conn.lock().unwrap();
    db::todos_list(&con)
}

fn main() {
  let conn = db::establish_connection();
    let state = AppState {
        conn: Mutex::new(db::establish_connection()),
    };

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![
      todos_list,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
