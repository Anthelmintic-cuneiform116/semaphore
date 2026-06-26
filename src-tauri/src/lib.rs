use std::sync::Arc;
use std::time::Duration;

use sem_core::config::Config;
use sem_core::ipc::{IpcServer, PruneTask};
use sem_core::state::{LightState, StateMachine};
use semctl::install;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};
use tokio::sync::RwLock;

#[derive(Clone, serde::Serialize)]
struct StatePayload {
    state: String,
}

#[tauri::command]
fn get_config() -> Config {
    Config::load()
}

#[tauri::command]
fn save_config(config: Config) -> Result<(), String> {
    config.save().map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_stealth(app: AppHandle, enabled: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window
            .set_content_protected(enabled)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn install_hooks(tool: String) -> Result<(), String> {
    let all = tool == "all";
    let tool_opt = if all { None } else { Some(tool.as_str()) };
    install::run_install(all, tool_opt).map_err(|e| e.to_string())
}

fn emit_state(app: &AppHandle, state: LightState) {
    let payload = StatePayload {
        state: match state {
            LightState::Green => "green".to_string(),
            LightState::Yellow => "yellow".to_string(),
            LightState::Red => "red".to_string(),
        },
    };
    let _ = app.emit("state-changed", payload);
}

fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Show Semaphore", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
    let stealth = MenuItem::with_id(app, "stealth", "Toggle Stealth", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &hide, &stealth, &quit])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "stealth" => {
                if let Some(window) = app.get_webview_window("main") {
                    let mut config = Config::load();
                    config.stealth = !config.stealth;
                    let _ = window.set_content_protected(config.stealth);
                    let _ = config.save();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .try_init()
        .ok();

    let config = Config::load();
    let machine = Arc::new(RwLock::new(StateMachine::new(Duration::from_secs(
        config.idle_timeout_secs,
    ))));
    let machine_ipc = Arc::clone(&machine);
    let machine_prune = Arc::clone(&machine);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            setup_tray(app.handle())?;

            let (server, handle) = IpcServer::new(machine_ipc);
            let prune = PruneTask::new(machine_prune, handle.state_tx.clone());
            let app_handle = app.handle().clone();
            let mut rx = handle.state_tx.subscribe();

            tauri::async_runtime::spawn(async move {
                if let Err(err) = server.run().await {
                    tracing::error!(?err, "ipc server failed");
                }
            });

            tauri::async_runtime::spawn(prune.run());

            tauri::async_runtime::spawn(async move {
                loop {
                    match rx.recv().await {
                        Ok(state) => emit_state(&app_handle, state),
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                        Err(_) => break,
                    }
                }
            });

            if let Some(window) = app.get_webview_window("main") {
                if config.stealth {
                    let _ = window.set_content_protected(true);
                }
                let _ = window.set_position(tauri::Position::Physical(
                    tauri::PhysicalPosition::new(config.window.x, config.window.y),
                ));
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            set_stealth,
            install_hooks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
