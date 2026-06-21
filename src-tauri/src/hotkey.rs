fn extract_from_cfg(cfg: &str, key: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(cfg)
        .ok()
        .and_then(|v| v[crate::constants::CFG_HOTKEYS][key].as_str().map(String::from))
        .filter(|s| !s.is_empty())
}

fn load_combo(key: &str, default_val: &str) -> String {
    let default = || default_val.to_string();
    let json = match crate::config::load_effective() {
        Ok(j) => j,
        Err(_) => return default(),
    };
    extract_from_cfg(&json, key).unwrap_or_else(default)
}

pub fn load_screenshot_combo() -> String {
    load_combo("screenshot", "Alt+W")
}

pub fn load_selection_combo() -> String {
    load_combo("selectionTranslate", "Alt+T")
}

fn normalize_shortcut_text(s: &str) -> String {
    s.replace(' ', "")
        .replace("Key", "")
        .replace("Digit", "")
        .replace("CommandOrControl", "Ctrl")
        .replace("Super", "Ctrl")
        .to_lowercase()
}

pub fn shortcut_equals(actual: &str, expected: &str) -> bool {
    normalize_shortcut_text(actual) == normalize_shortcut_text(expected)
}

pub fn try_register(app: &tauri::AppHandle, combo: &str, label: &str) {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;
    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
    use tauri::Emitter;
    match app.global_shortcut().register(combo) {
        Ok(_) => log::info!("registered {} shortcut: {}", label, combo),
        Err(_e) => {
            let msg = format!("快捷键「{}」({}) 已被其他程序占用", combo, label);
            log::warn!("{}", msg);
            let app_clone = app.clone();
            app.dialog()
                .message(&msg)
                .title("快捷键冲突")
                .kind(MessageDialogKind::Warning)
                .buttons(MessageDialogButtons::OkCancelCustom("去修改".to_string(), "忽略".to_string()))
                .show(move |ok| {
                    if ok {
                        let _ = app_clone.emit(crate::constants::EVENT_NAVIGATE, crate::constants::ROUTE_SETTINGS_HOTKEY);
                    }
                });
        }
    }
}

#[tauri::command]
pub fn reload_hotkeys(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    let _ = app.global_shortcut().unregister_all();

    let state = app.state::<crate::tray::ShortcutsEnabled>();
    if *state.0.lock().unwrap() {
        try_register(&app, &load_selection_combo(), "划词翻译");
        try_register(&app, &load_screenshot_combo(), "截图");
    }

    Ok(())
}
