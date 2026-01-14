// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
use core::{cmd, db, setup, window};
use tauri::{
    menu::{Menu, MenuItem, Submenu},
    Manager,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
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
            cmd::cmd_create_todo,
            cmd::cmd_get_todos,
            cmd::cmd_get_todo_detail,
            cmd::cmd_update_todo,
            cmd::cmd_delete_todo,
            cmd::cmd_get_status_counts,
            cmd::cmd_start_research,
            cmd::cmd_cancel_research,
            cmd::cmd_save_research_result,
            window::open_settings,
            window::open_todo_app,
        ])
        .setup(|app| {
            db::init_db(app.handle())?;

            let handle = app.handle();
            let todo_item =
                MenuItem::with_id(handle, "todo", "打开 Todo", true, Some("CmdOrCtrl+T"))?;
            let settings_item =
                MenuItem::with_id(handle, "settings", "设置", true, Some("CmdOrCtrl+,"))?;
            let reload_item =
                MenuItem::with_id(handle, "reload", "刷新", true, Some("CmdOrCtrl+R"))?;

            let app_submenu = Submenu::with_items(
                handle,
                "AI Todo",
                true,
                &[&todo_item, &settings_item, &reload_item],
            )?;

            let menu = Menu::with_items(handle, &[&app_submenu])?;
            app.set_menu(menu)?;

            app.on_menu_event(move |app_handle, event| match event.id().as_ref() {
                "todo" => {
                    window::open_todo_app(app_handle.clone());
                }
                "settings" => {
                    window::open_settings(app_handle.clone());
                }
                "reload" => {
                    if let Some(win) = app_handle.get_window("core") {
                        if let Some(webview) = win.get_webview("main") {
                            let _ = webview.eval("window.location.reload()");
                        }
                    }
                }
                _ => {}
            });

            setup::init(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running AI Todo application");
}
