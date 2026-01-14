// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod db;
mod models;

use core::{cmd, setup, window};
use tauri::{Manager, State};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            cmd::view_reload,
            cmd::view_url,
            cmd::view_go_forward,
            cmd::view_go_back,
            cmd::set_view_ask,
            cmd::get_app_conf,
            cmd::window_pin,
            cmd::ask_sync,
            cmd::ask_send,
            cmd::set_theme,
            window::open_settings,
            core::todo::create_todo,
            core::todo::update_todo,
            core::todo::delete_todo,
            core::todo::get_todo,
            core::todo::list_todos,
            core::todo::start_research,
            core::todo::get_research_results,
            core::todo::get_research_progress,
        ])
        .setup(|app| {
            let mut app_state = core::todo::AppState::new();
            let app_data_dir = app.path().app_data_dir().unwrap();
            std::fs::create_dir_all(&app_data_dir).unwrap();
            let db_path = app_data_dir.join("ai_todo.db");
            let db = db::Database::new(db_path).unwrap();
            
            app_state.init_db(db);
            
            app.manage(app_state);
            
            setup::init(app)
        })
        .run(tauri::generate_context!())
        .expect("error while running lencx/ChatGPT application");
}
