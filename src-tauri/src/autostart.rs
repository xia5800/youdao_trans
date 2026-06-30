#[cfg(windows)]
mod imp {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_SET_VALUE};
    use winreg::RegKey;

    const AL_REGKEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
    const TASK_MANAGER_OVERRIDE_REGKEY: &str =
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run";
    const TASK_MANAGER_OVERRIDE_ENABLED_VALUE: [u8; 12] = [
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    fn registry_value(app_path: &str) -> String {
        let path_quoted = if app_path.contains(' ') {
            format!("\"{}\"", app_path)
        } else {
            app_path.to_string()
        };
        format!("{} --autostart", path_quoted)
    }

    pub fn enable(app: &tauri::AppHandle) -> Result<(), String> {
        let exe =
            std::env::current_exe().map_err(|e| format!("failed to get exe path: {}", e))?;
        let app_path = exe.display().to_string();
        let value = registry_value(&app_path);

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let app_name = app.package_info().name.clone();

        hkcu.open_subkey_with_flags(AL_REGKEY, KEY_SET_VALUE)
            .map_err(|e| format!("failed to open Run key: {}", e))?
            .set_value(&app_name, &value)
            .map_err(|e| format!("failed to set Run value: {}", e))?;

        if let Ok(reg) =
            hkcu.open_subkey_with_flags(TASK_MANAGER_OVERRIDE_REGKEY, KEY_SET_VALUE)
        {
            let _ = reg.set_raw_value(
                &app_name,
                &winreg::RegValue {
                    vtype: winreg::enums::RegType::REG_BINARY,
                    bytes: TASK_MANAGER_OVERRIDE_ENABLED_VALUE.to_vec(),
                },
            );
        }

        Ok(())
    }

    pub fn disable(app: &tauri::AppHandle) -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let app_name = app.package_info().name.clone();

        hkcu.open_subkey_with_flags(AL_REGKEY, KEY_SET_VALUE)
            .map_err(|e| format!("failed to open Run key: {}", e))?
            .delete_value(&app_name)
            .map_err(|e| format!("failed to delete Run value: {}", e))?;

        Ok(())
    }

    pub fn is_enabled(app: &tauri::AppHandle) -> Result<bool, String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let app_name = app.package_info().name.clone();

        let al_enabled = hkcu
            .open_subkey_with_flags(AL_REGKEY, KEY_READ)
            .map_err(|e| format!("failed to open Run key: {}", e))?
            .get_value::<String, _>(&app_name)
            .is_ok();

        Ok(al_enabled)
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn enable(_app: &tauri::AppHandle) -> Result<(), String> {
        Err("autostart is only supported on Windows".into())
    }

    pub fn disable(_app: &tauri::AppHandle) -> Result<(), String> {
        Err("autostart is only supported on Windows".into())
    }

    pub fn is_enabled(_app: &tauri::AppHandle) -> Result<bool, String> {
        Err("autostart is only supported on Windows".into())
    }
}

#[tauri::command]
pub fn enable_autostart(app: tauri::AppHandle) -> Result<(), String> {
    imp::enable(&app)
}

#[tauri::command]
pub fn disable_autostart(app: tauri::AppHandle) -> Result<(), String> {
    imp::disable(&app)
}

#[tauri::command]
pub fn is_autostart_enabled(app: tauri::AppHandle) -> Result<bool, String> {
    imp::is_enabled(&app)
}
