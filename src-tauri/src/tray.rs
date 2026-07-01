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
                    if let Err(e) = app.emit(constants::EVENT_DISMISS_CLOSE_DIALOG, ()) {
                        log::error!("通知前端关闭对话框失败: {}", e);
                    }
                    if let Err(e) = app.emit(constants::EVENT_NAVIGATE, constants::ROUTE_TRANSLATE) {
                        log::error!("通知前端导航至翻译页面失败: {}", e);
                    }
                    window::show_main(app);
                }
                "screenshot_translate" => {
                    let app = app.clone();
                    if let Some(main) = app.get_webview_window(constants::WINDOW_MAIN) {
                        if let Err(e) = main.hide() {
                            log::error!("隐藏主窗口失败: {}", e);
                        }
                    }
                    tauri::async_runtime::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_millis(constants::SCREENSHOT_TRAY_DELAY_MS)).await;
                        if let Err(e) = capture::prepare_screenshot(app).await {
                            log::error!("托盘截图翻译失败: {}", e);
                        }
                    });
                }
                "dict_lookup" => {
                    if let Err(e) = app.emit(constants::EVENT_DISMISS_CLOSE_DIALOG, ()) {
                        log::error!("通知前端关闭对话框失败: {}", e);
                    }
                    if let Err(e) = app.emit(constants::EVENT_NAVIGATE, constants::ROUTE_DICTIONARY) {
                        log::error!("通知前端导航至词典页面失败: {}", e);
                    }
                    window::show_main(app);
                }
                "toggle_shortcuts" => {
                    use tauri_plugin_global_shortcut::GlobalShortcutExt;
                    let state = app.state::<ShortcutsEnabled>();
                    let mut enabled = match state.0.lock() {
                        Ok(e) => e,
                        Err(e) => {
                            log::error!("获取快捷键状态锁失败: {}", e);
                            return;
                        }
                    };
                    if *enabled {
                        if let Err(e) = app.global_shortcut().unregister_all() {
                            log::error!("注销全局快捷键失败: {}", e);
                        }
                        *enabled = false;
                        if let Err(e) = toggle_item_clone.set_text("启用快捷键") {
                            log::error!("更新托盘菜单文本失败: {}", e);
                        }
                        if let Err(e) = app.emit(constants::EVENT_SHORTCUTS_TOGGLED, false) {
                            log::error!("通知前端快捷键状态失败: {}", e);
                        }
                    } else {
                        hotkey::try_register(app, &hotkey::load_selection_combo(), "划词翻译");
                        hotkey::try_register(app, &hotkey::load_screenshot_combo(), "截图");
                        *enabled = true;
                        if let Err(e) = toggle_item_clone.set_text("停用快捷键") {
                            log::error!("更新托盘菜单文本失败: {}", e);
                        }
                        if let Err(e) = app.emit(constants::EVENT_SHORTCUTS_TOGGLED, true) {
                            log::error!("通知前端快捷键状态失败: {}", e);
                        }
                    }
                }
                "settings" => {
                    if let Err(e) = app.emit(constants::EVENT_DISMISS_CLOSE_DIALOG, ()) {
                        log::error!("通知前端关闭对话框失败: {}", e);
                    }
                    if let Err(e) = app.emit(constants::EVENT_NAVIGATE, constants::ROUTE_SETTINGS) {
                        log::error!("通知前端导航至设置页面失败: {}", e);
                    }
                    window::show_main(app);
                }
                "quit" => {
                    if let Some(overlay) = app.get_webview_window(constants::WINDOW_SCREENSHOT_OVERLAY) {
                        if let Err(e) = overlay.close() {
                            log::error!("关闭截图覆盖层失败: {}", e);
                        }
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
                if let Err(e) = tray.app_handle().emit(constants::EVENT_DISMISS_CLOSE_DIALOG, ()) {
                    log::error!("托盘点击通知前端失败: {}", e);
                }
                window::show_main(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}
