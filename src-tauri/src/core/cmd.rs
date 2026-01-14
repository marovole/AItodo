use tauri::{command, AppHandle, LogicalPosition, Manager, PhysicalSize};

use crate::core::{
    conf::AppConf,
    constant::{ASK_HEIGHT, TITLEBAR_HEIGHT},
    todo::{
        self, CreateTodoInput, StatusCounts, Todo, TodoStatus, TodoWithResearch, UpdateTodoInput,
    },
};

#[command]
pub fn view_reload(app: AppHandle) {
    app.get_window("core")
        .unwrap()
        .get_webview("main")
        .unwrap()
        .eval("window.location.reload()")
        .unwrap();
}

#[command]
pub fn view_url(app: AppHandle) -> tauri::Url {
    app.get_window("core")
        .unwrap()
        .get_webview("main")
        .unwrap()
        .url()
        .unwrap()
}

#[command]
pub fn view_go_forward(app: AppHandle) {
    app.get_window("core")
        .unwrap()
        .get_webview("main")
        .unwrap()
        .eval("window.history.forward()")
        .unwrap();
}

#[command]
pub fn view_go_back(app: AppHandle) {
    app.get_window("core")
        .unwrap()
        .get_webview("main")
        .unwrap()
        .eval("window.history.back()")
        .unwrap();
}

#[command]
pub fn window_pin(app: AppHandle, pin: bool) {
    let conf = AppConf::load(&app).unwrap();
    conf.amend(serde_json::json!({"stay_on_top": pin}))
        .unwrap()
        .save(&app)
        .unwrap();

    app.get_window("core")
        .unwrap()
        .set_always_on_top(pin)
        .unwrap();
}

#[command]
pub fn ask_sync(app: AppHandle, message: String) {
    app.get_window("core")
        .unwrap()
        .get_webview("main")
        .unwrap()
        .eval(&format!("ChatAsk.sync({})", message))
        .unwrap();
}

#[command]
pub fn ask_send(app: AppHandle) {
    let win = app.get_window("core").unwrap();

    win.get_webview("main")
        .unwrap()
        .eval(
            r#"
        ChatAsk.submit();
        setTimeout(() => {
            __TAURI__.webview.Webview.getByLabel('ask')?.setFocus();
        }, 500);
        "#,
        )
        .unwrap();
}

#[command]
pub fn set_theme(app: AppHandle, theme: String) {
    let conf = AppConf::load(&app).unwrap();
    conf.amend(serde_json::json!({"theme": theme}))
        .unwrap()
        .save(&app)
        .unwrap();

    app.restart();
}

#[command]
pub fn get_app_conf(app: AppHandle) -> AppConf {
    AppConf::load(&app).unwrap()
}

#[command]
pub fn cmd_create_todo(input: CreateTodoInput) -> Result<Todo, String> {
    todo::create_todo(input).map_err(|e| e.to_string())
}

#[command]
pub fn cmd_get_todos(status: Option<String>) -> Result<Vec<Todo>, String> {
    todo::get_todos(status.as_deref()).map_err(|e| e.to_string())
}

#[command]
pub fn cmd_get_todo_detail(id: String) -> Result<Option<TodoWithResearch>, String> {
    todo::get_todo_with_research(&id).map_err(|e| e.to_string())
}

#[command]
pub fn cmd_update_todo(id: String, input: UpdateTodoInput) -> Result<Option<Todo>, String> {
    todo::update_todo(&id, input).map_err(|e| e.to_string())
}

#[command]
pub fn cmd_delete_todo(id: String) -> Result<bool, String> {
    todo::delete_todo(&id).map_err(|e| e.to_string())
}

#[command]
pub fn cmd_get_status_counts() -> Result<StatusCounts, String> {
    todo::get_status_counts().map_err(|e| e.to_string())
}

#[command]
pub fn cmd_start_research(app: AppHandle, id: String) -> Result<Option<Todo>, String> {
    let todo = todo::get_todo_by_id(&id).map_err(|e| e.to_string())?;

    if let Some(t) = todo {
        let input = UpdateTodoInput {
            title: None,
            description: None,
            url: None,
            status: Some(TodoStatus::Researching),
        };
        let updated = todo::update_todo(&id, input).map_err(|e| e.to_string())?;

        // Trigger research in the main webview
        if let Some(win) = app.get_window("core") {
            if let Some(webview) = win.get_webview("main") {
                let prompt = format!(
                    "Please research: {}. Context: {}",
                    t.title,
                    t.description
                        .unwrap_or_else(|| "No description provided".to_string())
                );
                // We need to escape the prompt for JS string injection
                let escaped_prompt = prompt.replace("'", "\\'").replace("\n", "\\n");
                let script = format!("window.DeepResearch.start('{}', '{}')", id, escaped_prompt);
                let _ = webview.eval(&script);
            }
        }

        Ok(updated)
    } else {
        Ok(None)
    }
}

#[command]
pub fn cmd_cancel_research(app: AppHandle, id: String) -> Result<Option<Todo>, String> {
    let input = UpdateTodoInput {
        title: None,
        description: None,
        url: None,
        status: Some(TodoStatus::Pending),
    };
    let updated = todo::update_todo(&id, input).map_err(|e| e.to_string())?;

    if let Some(win) = app.get_window("core") {
        if let Some(webview) = win.get_webview("main") {
            let _ = webview.eval("window.DeepResearch.cancel()");
        }
    }

    Ok(updated)
}

#[command]
pub fn cmd_save_research_result(
    todo_id: String,
    source: String,
    content: String,
    raw_html: Option<String>,
    started_at: String,
) -> Result<todo::ResearchResult, String> {
    todo::save_research_result(
        &todo_id,
        &source,
        &content,
        raw_html.as_deref(),
        &started_at,
    )
    .map_err(|e| e.to_string())
}

#[command]
pub fn set_view_ask(app: AppHandle, enabled: bool) {
    let conf = AppConf::load(&app).unwrap();
    conf.amend(serde_json::json!({"ask_mode": enabled}))
        .unwrap()
        .save(&app)
        .unwrap();

    let core_window = app.get_window("core").unwrap();
    let ask_mode_height = if enabled { ASK_HEIGHT } else { 0.0 };
    let scale_factor = core_window.scale_factor().unwrap();
    let titlebar_height = (scale_factor * TITLEBAR_HEIGHT).round() as u32;
    let win_size = core_window.inner_size().unwrap();
    let ask_height = (scale_factor * ask_mode_height).round() as u32;

    let main_view = core_window.get_webview("main").unwrap();
    let titlebar_view = core_window.get_webview("titlebar").unwrap();
    let ask_view = core_window.get_webview("ask").unwrap();

    if enabled {
        ask_view.set_focus().unwrap();
    } else {
        main_view.set_focus().unwrap();
    }

    let set_view_properties =
        |view: &tauri::Webview, position: LogicalPosition<f64>, size: PhysicalSize<u32>| {
            if let Err(e) = view.set_position(position) {
                eprintln!("[cmd:view:position] Failed to set view position: {}", e);
            }
            if let Err(e) = view.set_size(size) {
                eprintln!("[cmd:view:size] Failed to set view size: {}", e);
            }
        };

    #[cfg(target_os = "macos")]
    {
        set_view_properties(
            &main_view,
            LogicalPosition::new(0.0, TITLEBAR_HEIGHT),
            PhysicalSize::new(
                win_size.width,
                win_size.height - (titlebar_height + ask_height),
            ),
        );
        set_view_properties(
            &titlebar_view,
            LogicalPosition::new(0.0, 0.0),
            PhysicalSize::new(win_size.width, titlebar_height),
        );
        set_view_properties(
            &ask_view,
            LogicalPosition::new(
                0.0,
                (win_size.height as f64 / scale_factor) - ask_mode_height,
            ),
            PhysicalSize::new(win_size.width, ask_height),
        );
    }

    #[cfg(not(target_os = "macos"))]
    {
        set_view_properties(
            &main_view,
            LogicalPosition::new(0.0, 0.0),
            PhysicalSize::new(
                win_size.width,
                win_size.height - (ask_height + titlebar_height),
            ),
        );
        set_view_properties(
            &titlebar_view,
            LogicalPosition::new(
                0.0,
                (win_size.height as f64 / scale_factor) - TITLEBAR_HEIGHT,
            ),
            PhysicalSize::new(win_size.width, titlebar_height),
        );
        set_view_properties(
            &ask_view,
            LogicalPosition::new(
                0.0,
                (win_size.height as f64 / scale_factor) - ask_mode_height - TITLEBAR_HEIGHT,
            ),
            PhysicalSize::new(win_size.width, ask_height),
        );
    }
}
