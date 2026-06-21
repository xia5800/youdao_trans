use crate::{capture, constants, hotkey, window};
use std::sync::Mutex;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

pub struct ShortcutsEnabled(pub Mutex<bool>);

pub fn build(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let input_item = MenuItemBuilder::with_id("input_translate", "输入翻译").build(app)?;
    let screenshot_item =
        MenuItemBuilder::with_id("screenshot_translate", "截图翻译").build(app)?;
    let dict_item = MenuItemBuilder::with_id("dict_lookup", "查单词").build(app)?;
    let toggle_item = MenuItemBuilder::with_id("toggle_shortcuts", "停用快捷键").build(app)?;
    let toggle_item_clone = toggle_item.clone();
    let settings_item = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&input_item)
        .item(&screenshot_item)
        .item(&dict_item)
        .separator()
        .item(&toggle_item)
        .separator()
        .item(&settings_item)
        .separator()
        .item(&quit_item)
        .build()?;

    let icon = app.default_window_icon().cloned().unwrap();

    let _tray_icon = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .tooltip(constants::APP_NAME)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| {
            match event.id.as_ref() {
                "input_translate" => {
                    let _ = app.emit(constants::EVENT_DISMISS_CLOSE_DIALOG, ());
                    let _ = app.emit(constants::EVENT_NAVIGATE, constants::ROUTE_TRANSLATE);
                    window::show_main(app);
                }
                "screenshot_translate" => {
                    let app = app.clone();
                    if let Some(main) = app.get_webview_window(constants::WINDOW_MAIN) {
                        let _ = main.hide();
                    }
                    tauri::async_runtime::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_millis(constants::SCREENSHOT_TRAY_DELAY_MS)).await;
                        let _ = capture::prepare_screenshot(app).await;
                    });
                }
                "dict_lookup" => {
                    let _ = app.emit(constants::EVENT_DISMISS_CLOSE_DIALOG, ());
                    let _ = app.emit(constants::EVENT_NAVIGATE, constants::ROUTE_DICTIONARY);
                    window::show_main(app);
                }
                "toggle_shortcuts" => {
                    use tauri_plugin_global_shortcut::GlobalShortcutExt;
                    let state = app.state::<ShortcutsEnabled>();
                    let mut enabled = state.0.lock().unwrap();
                    if *enabled {
                        let _ = app.global_shortcut().unregister_all();
                        *enabled = false;
                        let _ = toggle_item_clone.set_text("启用快捷键");
                        let _ = app.emit(constants::EVENT_SHORTCUTS_TOGGLED, false);
                    } else {
                        hotkey::try_register(app, &hotkey::load_selection_combo(), "划词翻译");
                        hotkey::try_register(app, &hotkey::load_screenshot_combo(), "截图");
                        *enabled = true;
                        let _ = toggle_item_clone.set_text("停用快捷键");
                        let _ = app.emit(constants::EVENT_SHORTCUTS_TOGGLED, true);
                    }
                }
                "settings" => {
                    let _ = app.emit(constants::EVENT_DISMISS_CLOSE_DIALOG, ());
                    let _ = app.emit(constants::EVENT_NAVIGATE, constants::ROUTE_SETTINGS);
                    window::show_main(app);
                }
                "quit" => {
                    if let Some(overlay) = app.get_webview_window(constants::WINDOW_SCREENSHOT_OVERLAY) {
                        let _ = overlay.close();
                    }
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                ..
            } = event
            {
                let _ = tray.app_handle().emit(constants::EVENT_DISMISS_CLOSE_DIALOG, ());
                window::show_main(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}
