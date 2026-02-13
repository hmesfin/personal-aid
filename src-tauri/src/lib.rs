use std::sync::Mutex;
use std::time::Duration;

use tauri::Manager;
use tauri_plugin_shell::process::CommandChild;
use tauri_plugin_shell::ShellExt;

const SIDECAR_PORT: u16 = 18008;

/// Holds the running sidecar child process.
struct SidecarState(Mutex<Option<CommandChild>>);

#[tauri::command]
async fn start_sidecar(app: tauri::AppHandle) -> Result<String, String> {
    let state = app.state::<SidecarState>();
    let mut child_lock = state.0.lock().map_err(|e| e.to_string())?;

    if child_lock.is_some() {
        return Ok("Sidecar already running".into());
    }

    let sidecar = app
        .shell()
        .sidecar("personal-aid-api")
        .map_err(|e| format!("Failed to create sidecar command: {e}"))?
        .args([SIDECAR_PORT.to_string()]);

    let (_, child) = sidecar
        .spawn()
        .map_err(|e| format!("Failed to spawn sidecar: {e}"))?;

    *child_lock = Some(child);

    Ok("Sidecar started".into())
}

#[tauri::command]
async fn stop_sidecar(app: tauri::AppHandle) -> Result<String, String> {
    let state = app.state::<SidecarState>();
    let mut child_lock = state.0.lock().map_err(|e| e.to_string())?;

    if let Some(child) = child_lock.take() {
        child.kill().map_err(|e| format!("Failed to kill sidecar: {e}"))?;
        Ok("Sidecar stopped".into())
    } else {
        Ok("Sidecar was not running".into())
    }
}

#[tauri::command]
async fn check_sidecar_health() -> Result<serde_json::Value, String> {
    let url = format!("http://127.0.0.1:{SIDECAR_PORT}/api/health");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Health check failed: {e}"))?;

    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse health response: {e}"))?;

    Ok(body)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(SidecarState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            start_sidecar,
            stop_sidecar,
            check_sidecar_health,
        ])
        .setup(|app| {
            // Auto-start the sidecar on app launch.
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = start_sidecar(handle).await {
                    eprintln!("Failed to auto-start sidecar: {e}");
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            // Auto-stop sidecar when the main window is destroyed.
            if let tauri::WindowEvent::Destroyed = event {
                let app = window.app_handle().clone();
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = stop_sidecar(app).await {
                        eprintln!("Failed to stop sidecar on exit: {e}");
                    }
                });
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
