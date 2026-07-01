mod autostart;
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
mod tts;
mod util;
mod window;

use tauri::{Listener, Manager};

fn init_logger() {
    let log_dir = crate::config::default_logs_dir_inner();

    let _ = std::fs::create_dir_all(&log_dir);

    cleanup_old_logs(&log_dir, 7_usize);

    let file_path = log_dir.join(format!(
        "app_{}.log",
        chrono::Local::now().format("%Y-%m-%d")
    ));

    let file = match fern::log_file(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("打开日志文件失败 {}: {}", file_path.display(), e);
            return;
        }
    };

    let dispatch_base = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .chain(file);

    #[cfg(debug_assertions)]
    let dispatch = fern::Dispatch::new()
        .chain(
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}] [{}] {} - {}",
                        chrono::Local::now().format("%H:%M:%S"),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .level(log::LevelFilter::Info)
                .chain(std::io::stdout()),
        )
        .chain(dispatch_base);

    #[cfg(not(debug_assertions))]
    let dispatch = dispatch_base;

    if let Err(e) = dispatch.apply() {
        eprintln!("logger already set: {}", e);
    }
}

fn cleanup_old_logs(dir: &std::path::Path, keep_count: usize) {
    let Ok(mut entries) = std::fs::read_dir(dir).map(|e| e.flatten().filter_map(|e| {
        let path = e.path();
        let name = path.file_stem().and_then(|s| s.to_str())?;
        if name.starts_with("app_") { Some(path) } else { None }
    }).collect::<Vec<_>>()) else { return };
    entries.sort();
    if entries.len() > keep_count {
        for old in entries.iter().take(entries.len() - keep_count) {
            let _ = std::fs::remove_file(old);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize rustls with ring provider (avoids conflict with aws-lc-rs)
    let _ = rustls::crypto::ring::default_provider().install_default();

    init_logger();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            log::info!("收到单实例通知，显示主窗口");
            window::show_main(app);
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
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
                                if let Err(e) = main.hide() {
                                    log::error!("快捷键触发的隐藏主窗口失败: {}", e);
                                }
                            }
                        }

                        tauri::async_runtime::spawn(async move {
                            let selection_combo = hotkey::load_selection_combo();
                            if is_screenshot {
                                if let Err(e) = capture::prepare_screenshot(app.clone()).await {
                                    log::error!("快捷键截图翻译失败: {}", e);
                                }
                            } else if hotkey::shortcut_equals(&shortcut_str, &selection_combo) {
                                selection::handle_selection_translate(app).await;
                            }
                        });
                    }
                })
                .build(),
        )
        .setup(|app| {
            let is_autostart = std::env::args().any(|arg| arg == "--autostart");
            app.manage(window::AutostartLaunched(std::sync::Mutex::new(is_autostart)));

            app.manage(selection::SelectionState(std::sync::Mutex::new(None)));
            app.manage(ocr::PendingOcrState(std::sync::Mutex::new(None)));
            app.manage(tray::ShortcutsEnabled(std::sync::Mutex::new(true)));
            app.manage(dict::DictDownloadState::new());

            let window = app.get_webview_window("main")
                .expect("main window should exist in setup");

            if let Err(e) = tray::build(app.handle()) {
                log::error!("构建系统托盘失败: {}", e);
            }

            if let Some((cx, cy)) = window::cursor_position() {
                match window.available_monitors() {
                    Ok(monitors) => {
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
                                if let Err(e) = window.set_position(tauri::PhysicalPosition::new(
                                    wx.max(p.x),
                                    wy.max(p.y),
                                )) {
                                    log::error!("设置窗口位置失败: {}", e);
                                }
                                break;
                            }
                        }
                    }
                    Err(e) => log::error!("获取显示器信息失败: {}", e),
                }
            }

            if let Err(e) = window.set_focus() {
                log::error!("设置窗口焦点失败: {}", e);
            }

            capture::cleanup_screenshot_file();

            let _ = app.handle().listen(constants::EVENT_SCREENSHOT_READY, |_event| {
                capture::FRONTEND_READY.store(true, std::sync::atomic::Ordering::SeqCst);
            });

            if let Err(e) = tauri::WebviewWindowBuilder::new(
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
            .build()
            {
                log::error!("创建截图覆盖层窗口失败: {}", e);
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    if let Err(e) = window.hide() {
                        log::error!("隐藏主窗口失败: {}", e);
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            window::is_autostart_launched,
            autostart::enable_autostart,
            autostart::disable_autostart,
            autostart::is_autostart_enabled,
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
            dict::check_dict_db,
            dict::download_dict_db,
            dict::pause_dict_download,
            dict::resume_dict_download,
            dict::cancel_dict_download,
            history::save_history,
            history::load_history,
            history::delete_history,
            history::delete_history_batch,
            history::delete_history_all,
            history::toggle_history_favorite,
            tts::tts_speak,
            tts::tts_list_voices,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
