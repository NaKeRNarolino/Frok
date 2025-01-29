use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

#[tauri::command]
pub fn get_windows_accent_color() -> [u8; 3] {
    let hkey_current_user =
        match RegKey::predef(HKEY_CURRENT_USER).open_subkey("SOFTWARE\\Microsoft\\Windows\\DWM") {
            Ok(key) => key,
            Err(_) => return [0, 255, 0],
        };

    let color = match hkey_current_user.get_value::<u32, _>("AccentColor") {
        Ok(value) => value,
        Err(_) => return [0, 255, 0],
    };

    let r = (color & 0xff) as u8;
    let g = ((color >> 8) & 0xff) as u8;
    let b = ((color >> 16) & 0xff) as u8;

    [r, g, b]
}
