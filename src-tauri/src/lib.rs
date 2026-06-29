mod capture;
mod config;
mod constants;
mod dict;
mod download;
mod history;
mod hotkey;
mod ocr;
mod selection;
mod translate;
mod tray;
mod window;

use tauri::{Listener, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            window::show_main(app);
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        let app = app.clone();
                        let shortcut_str = shortcut.to_string();

                        let screenshot_combo = hotkey::load_screenshot_combo();
                        let is_screenshot = hotkey::shortcut_equals(&shortcut_str, &screenshot_combo);
                        if is_screenshot {
                            if let Some(main) = app.get_webview_window(constants::WINDOW_MAIN) {
                                let _ = main.hide();
                            }
                        }

                        tauri::async_runtime::spawn(async move {
                            let selection_combo = hotkey::load_selection_combo();
                            if is_screenshot {
                                let _ = capture::prepare_screenshot(app.clone()).await;
                            } else if hotkey::shortcut_equals(&shortcut_str, &selection_combo) {
                                selection::handle_selection_translate(app).await;
                            }
                        });
                    }
                })
                .build(),
        )
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let is_autostart = std::env::args().any(|arg| arg == "--autostart");
            app.manage(window::AutostartLaunched(std::sync::Mutex::new(is_autostart)));

            app.manage(selection::SelectionState(std::sync::Mutex::new(None)));
            app.manage(ocr::PendingOcrState(std::sync::Mutex::new(None)));
            app.manage(tray::ShortcutsEnabled(std::sync::Mutex::new(true)));

            let window = app.get_webview_window("main").unwrap();

            let _ = tray::build(app.handle());

            // Center window on the monitor where the mouse cursor is
            if let Some((cx, cy)) = window::cursor_position() {
                if let Ok(monitors) = window.available_monitors() {
                    for m in &monitors {
                        let p = m.position();
                        let s = m.size();
                        if cx >= p.x
                            && cx < p.x + s.width as i32
                            && cy >= p.y
                            && cy < p.y + s.height as i32
                        {
                            let ws = window.outer_size().unwrap_or_default();
                            let wx = p.x + (s.width as i32 - ws.width as i32) / 2;
                            let wy = p.y + (s.height as i32 - ws.height as i32) / 2;
                            let _ = window.set_position(tauri::PhysicalPosition::new(
                                wx.max(p.x),
                                wy.max(p.y),
                            ));
                            break;
                        }
                    }
                }
            }

            let _ = window.set_focus();

            capture::cleanup_screenshot_file();

            let _ = app.handle().listen(constants::EVENT_SCREENSHOT_READY, |_event| {
                capture::FRONTEND_READY.store(true, std::sync::atomic::Ordering::SeqCst);
            });

            let _ = tauri::WebviewWindowBuilder::new(
                app.handle(),
                constants::WINDOW_SCREENSHOT_OVERLAY,
                tauri::WebviewUrl::App(constants::SCREENSHOT_HTML.into()),
            )
            .always_on_top(true)
            .decorations(false)
            .transparent(true)
            .visible(false)
            .skip_taskbar(true)
            .fullscreen(true)
            .build();

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            window::is_autostart_launched,
            window::app_exit,
            window::set_window_bg,
            window::minimize_window,
            window::maximize_window,
            window::close_window,
            window::pin_window,
            window::is_window_maximized,
            window::is_window_pinned,
            window::hide_window,
            window::show_window,
            config::load_config,
            config::save_config,
            config::config_filename,
            config::default_config_dir,
            config::default_db_dir,
            config::remove_config,
            hotkey::reload_hotkeys,
            selection::get_selection_payload,
            capture::prepare_screenshot,
            capture::cancel_overlay,
            capture::cleanup_screenshot,
            ocr::take_pending_ocr,
            ocr::finish_ocr_screenshot,
            ocr::ocr_command,
            ocr::check_ocr_models_state,
            ocr::ocr_models_data_dir,
            ocr::download_ocr_models,
            ocr::retry_download_ocr_file,
            translate::translate_command,
            dict::dict_suggestions,
            dict::dict_lookup,
            history::save_history,
            history::load_history,
            history::delete_history,
            history::delete_history_batch,
            history::delete_history_all,
            history::toggle_history_favorite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
