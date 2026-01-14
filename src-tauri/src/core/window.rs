use tauri::{command, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::core::constant::{WINDOW_SETTINGS, WINDOW_TODO};

#[command]
pub fn open_settings(app: AppHandle) {
    match app.get_webview_window(WINDOW_SETTINGS) {
        Some(window) => {
            window.show().unwrap();
        }
        None => {
            WebviewWindowBuilder::new(&app, WINDOW_SETTINGS, WebviewUrl::App("index.html".into()))
                .build()
                .unwrap();
        }
    }
}

#[command]
pub fn open_todo_app(app: AppHandle) {
    match app.get_webview_window(WINDOW_TODO) {
        Some(window) => {
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        None => {
            WebviewWindowBuilder::new(&app, "todo", WebviewUrl::App("index.html".into()))
                .title("AI Todo")
                .inner_size(1000.0, 700.0)
                .min_inner_size(600.0, 400.0)
                .build()
                .unwrap();
        }
    }
}
