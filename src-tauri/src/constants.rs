pub const WINDOW_MAIN: &str = "main";
pub const WINDOW_SCREENSHOT_OVERLAY: &str = "screenshot-overlay";
pub const WINDOW_SELECTION_POPUP: &str = "selection-popup";

pub const EVENT_NAVIGATE: &str = "navigate";
pub const EVENT_OCR_RESULT: &str = "ocr-result";
pub const EVENT_OCR_ERROR: &str = "ocr-error";
pub const EVENT_CHECK_PENDING_OCR: &str = "check-pending-ocr";
pub const EVENT_SHORTCUTS_TOGGLED: &str = "shortcuts-toggled";
pub const EVENT_SELECTION_UPDATE: &str = "selection-update";
pub const EVENT_SELECTION_TRANSLATE_ERROR: &str = "selection-translate-error";
pub const EVENT_DISMISS_CLOSE_DIALOG: &str = "dismiss-close-dialog";
pub const EVENT_CAPTURE_SCREEN: &str = "capture-screen";
pub const EVENT_SCREENSHOT_READY: &str = "screenshot-ready";

pub const EVENT_DOWNLOAD_START: &str = "download-start";
pub const EVENT_DOWNLOAD_PROGRESS: &str = "download-progress";
pub const EVENT_DOWNLOAD_COMPLETE: &str = "download-complete";
pub const EVENT_DOWNLOAD_ERROR: &str = "download-error";
pub const ROUTE_TRANSLATE: &str = "/translate";
pub const ROUTE_DICTIONARY: &str = "/dictionary";
pub const ROUTE_SETTINGS: &str = "/settings";
pub const ROUTE_SETTINGS_HOTKEY: &str = "/settings/hotkey";

pub const APP_NAME: &str = "优道翻译";
pub const APP_DATA_DIR: &str = "youdao-fanyi";

pub const SCREENSHOT_HTML: &str = "screenshot.html";
pub const POPUP_HTML: &str = "popup.html";

pub const CFG_CONFIG_PATH: &str = "configPath";
pub const CFG_HOTKEYS: &str = "hotkeys";
pub const CFG_ACTIVE_OCR: &str = "activeOcr";
pub const CFG_OCR_KEYS: &str = "ocrKeys";

pub const DEFAULT_DELAY_TIME_MS: u64 = 600;
pub const DEFAULT_AUTO_TRANSLATE_DELAY_MS: u64 = 500;

pub const TOKEN_CACHE_TTL_SECS: u64 = 480;

pub const SCREENSHOT_TRAY_DELAY_MS: u64 = 200;
pub const KEY_SIM_DELAY_MS: u64 = 20;

pub const SELECTION_RETRY_COUNT: u32 = 3;
pub const DICT_DEFAULT_SUGGESTION_LIMIT: u32 = 10;
