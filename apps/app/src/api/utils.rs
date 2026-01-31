use serde::{Deserialize, Serialize};
use theseus::{
    handler,
    prelude::{CommandPayload, DirectoryInfo},
};

use crate::api::Result;
use dashmap::DashMap;
use std::path::PathBuf;

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("utils")
        .invoke_handler(tauri::generate_handler![
            get_os,
            highlight_in_folder,
            open_path,
            show_launcher_logs_folder,
            progress_bars_list,
            get_opening_command
        ])
        .build()
}

/// Gets OS
#[tauri::command]
pub fn get_os() -> OS {
    OS::Linux
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OS {
    Linux,
}

// Lists active progress bars
// Create a new HashMap with the same keys
// Values provided should not be used directly, as they are not guaranteed to be up-to-date
#[tauri::command]
pub async fn progress_bars_list(
) -> Result<DashMap<uuid::Uuid, theseus::LoadingBar>> {
    let res = theseus::EventState::list_progress_bars().await?;
    Ok(res)
}

#[tauri::command]
pub fn highlight_in_folder(path: PathBuf) {
    let res = opener::reveal(path);

    if let Err(e) = res {
        tracing::error!("Failed to highlight file in folder: {}", e);
    }
}

#[tauri::command]
pub fn open_path(path: PathBuf) {
    let res = opener::open(path);

    if let Err(e) = res {
        tracing::error!("Failed to open path: {}", e);
    }
}

#[tauri::command]
pub fn show_launcher_logs_folder() {
    let path = DirectoryInfo::launcher_logs_dir().unwrap_or_default();
    // failure to get folder just opens filesystem
    // (ie: if in debug mode only and launcher_logs never created)
    open_path(path);
}

// Get opening command
// For example, if a user clicks on an .mrpack to open the app.
// This should be called once and only when the app is done booting up and ready to receive a command
// Returns a Command struct- see events.js
#[tauri::command]
pub async fn get_opening_command() -> Result<Option<CommandPayload>> {
    // Tauri is not CLI, we use arguments as path to file to call
    let cmd_arg = std::env::args_os().nth(1);

    tracing::info!("opening command {cmd_arg:?}");

    let cmd_arg = cmd_arg.map(|path| path.to_string_lossy().to_string());
    if let Some(cmd) = cmd_arg {
        tracing::debug!("Opening command: {:?}", cmd);
        return Ok(Some(handler::parse_command(&cmd).await?));
    }
    Ok(None)
}

// helper function called when redirected by a weblink (ie: modrith://do-something) or when redirected by a .mrpack file (in which case its a filepath)
// We hijack the deep link library (which also contains functionality for instance-checking)
pub async fn handle_command(command: String) -> Result<()> {
    tracing::info!("handle command: {command}");
    Ok(theseus::handler::parse_and_emit_command(&command).await?)
}
