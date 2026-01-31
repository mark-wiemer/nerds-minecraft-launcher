use native_dialog::{MessageDialog, MessageType};
use tauri::{Listener, Manager};
use theseus::prelude::*;

mod api;
mod error;

// Should be called in launcher initialization
#[tracing::instrument(skip_all)]
#[tauri::command]
async fn initialize_state(app: tauri::AppHandle) -> api::Result<()> {
    tracing::info!("Initializing app event state...");
    theseus::EventState::init(app.clone()).await?;

    #[cfg(feature = "updater")]
    {
        use tauri_plugin_updater::UpdaterExt;

        let updater = app.updater_builder().build()?;

        let update_fut = updater.check();

        tracing::info!("Initializing app state...");
        State::init().await?;

        let check_bar = theseus::init_loading(
            theseus::LoadingBarType::CheckingForUpdates,
            1.0,
            "Checking for updates...",
        )
        .await?;

        tracing::info!("Checking for updates...");
        let update = update_fut.await;

        drop(check_bar);

        if let Some(update) = update.ok().flatten() {
            tracing::info!("Update found: {:?}", update.download_url);
            let loader_bar_id = theseus::init_loading(
                theseus::LoadingBarType::LauncherUpdate {
                    version: update.version.clone(),
                    current_version: update.current_version.clone(),
                },
                1.0,
                "Updating Modrinth App...",
            )
            .await?;

            // 100 MiB
            const DEFAULT_CONTENT_LENGTH: u64 = 1024 * 1024 * 100;

            update
                .download_and_install(
                    |chunk_length, content_length| {
                        let _ = theseus::emit_loading(
                            &loader_bar_id,
                            (chunk_length as f64)
                                / (content_length
                                    .unwrap_or(DEFAULT_CONTENT_LENGTH)
                                    as f64),
                            None,
                        );
                    },
                    || {},
                )
                .await?;

            app.restart();
        }
    }

    #[cfg(not(feature = "updater"))]
    {
        State::init().await?;
    }

    tracing::info!("Finished checking for updates!");
    let state = State::get().await?;
    app.asset_protocol_scope()
        .allow_directory(state.directories.caches_dir(), true)?;
    app.asset_protocol_scope()
        .allow_directory(state.directories.caches_dir().join("icons"), true)?;

    Ok(())
}

// Should be call once Vue has mounted the app
#[tracing::instrument(skip_all)]
#[tauri::command]
fn show_window(app: tauri::AppHandle) {
    let win = app.get_window("main").unwrap();
    if let Err(e) = win.show() {
        MessageDialog::new()
            .set_type(MessageType::Error)
            .set_title("Initialization error")
            .set_text(&format!(
                "Cannot display application window due to an error:\n{}",
                e
            ))
            .show_alert()
            .unwrap();
        panic!("cannot display application window")
    } else {
        let _ = win.set_focus();
    }
}

#[tauri::command]
fn is_dev() -> bool {
    cfg!(debug_assertions)
}

// Toggles decorations
#[tauri::command]
async fn toggle_decorations(b: bool, window: tauri::Window) -> api::Result<()> {
    window.set_decorations(b).map_err(|e| {
        theseus::Error::from(theseus::ErrorKind::OtherError(format!(
            "Failed to toggle decorations: {}",
            e
        )))
    })?;
    Ok(())
}

#[tauri::command]
fn restart_app(app: tauri::AppHandle) {
    app.restart();
}

// if Tauri app is called with arguments, then those arguments will be treated as commands
// ie: deep links or filepaths for .mrpacks
fn main() {
    /*
        tracing is set basd on the environment variable RUST_LOG=xxx, depending on the amount of logs to show
            ERROR > WARN > INFO > DEBUG > TRACE
        eg. RUST_LOG=info will show info, warn, and error logs
            RUST_LOG="theseus=trace" will show *all* messages but from theseus only (and not dependencies using similar crates)
            RUST_LOG="theseus=trace" will show *all* messages but from theseus only (and not dependencies using similar crates)

        Error messages returned to Tauri will display as traced error logs if they return an error.
        This will also include an attached span trace if the error is from a tracing error, and the level is set to info, debug, or trace

        on unix:
            RUST_LOG="theseus=trace" {run command}

    */
    let _log_guard = theseus::start_logger();

    tracing::info!("Initialized tracing subscriber. Loading Modrinth App!");

    let mut builder = tauri::Builder::default();

    #[cfg(feature = "updater")]
    {
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
    }

    builder = builder
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if let Some(payload) = args.get(1) {
                tracing::info!("Handling deep link from arg {payload}");
                let payload = payload.clone();
                tauri::async_runtime::spawn(api::utils::handle_command(
                    payload,
                ));
            }

            if let Some(win) = app.get_window("main") {
                let _ = win.set_focus();
            }
        }))
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_filename("app-window-state.json")
                .build(),
        )
        .setup(|app| {
            app.listen("deep-link://new-url", |url| {
                let payload = url.payload().to_owned();
                tracing::info!("Handling deep link {payload}");
                tauri::async_runtime::spawn(api::utils::handle_command(
                    payload,
                ));
                dbg!(url);
            });

            Ok(())
        });

    builder = builder
        .plugin(api::auth::init())
        .plugin(api::mr_auth::init())
        .plugin(api::import::init())
        .plugin(api::logs::init())
        .plugin(api::jre::init())
        .plugin(api::metadata::init())
        .plugin(api::pack::init())
        .plugin(api::process::init())
        .plugin(api::profile::init())
        .plugin(api::profile_create::init())
        .plugin(api::settings::init())
        .plugin(api::tags::init())
        .plugin(api::utils::init())
        .plugin(api::cache::init())
        .plugin(api::ads::init())
        .plugin(api::friends::init())
        .invoke_handler(tauri::generate_handler![
            initialize_state,
            is_dev,
            toggle_decorations,
            show_window,
            restart_app,
        ]);

    tracing::info!("Initializing app...");
    let app = builder.build(tauri::generate_context!());

    match app {
        Ok(app) => {
            app.run(|_app, _event| {});
        }
        Err(e) => {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Initialization error")
                .set_text(&format!(
                    "Cannot initialize application due to an error:\n{:?}",
                    e
                ))
                .show_alert()
                .unwrap();

            tracing::error!("Error while running tauri application: {:?}", e);
            panic!("{1}: {:?}", e, "error while running tauri application")
        }
    }
}
