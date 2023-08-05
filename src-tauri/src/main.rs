// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use schema::todos;
use std::error::Error;
use std::{error, string, sync::Mutex};

// Start of DB example
// use super::db::{};
use diesel_migrations::{embed_migrations, EmbedMigrations};

#[macro_use]
extern crate diesel;
#[macro_use] 
extern crate diesel_migrations;
embed_migrations!("./migrations/");
use tauri::Manager;
use tauri::api::path;

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
  
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      todos_list
    ])
    .setup(|app|{
      let path = path::app_data_dir(&app.config()).unwrap();
      let str = path.to_str().unwrap();
      let db_path = format!("{}{}", str, "/store.sqlite"); 
      let conn = db::establish_connection(&db_path);
      diesel_migrations::run_pending_migrations(&conn).expect("Error migrating");
      let state = AppState {
        conn: Mutex::new(conn),
      };
      app.manage(state);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
