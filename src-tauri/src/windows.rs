use std::ffi::OsStr;
use std::os::windows::prelude::OsStrExt;
use windows::core::PWSTR;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::Foundation::{HWND, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;

fn _open_file_with_selector(file_path: &str) {
    let wide_path: Vec<u16> = OsStr::new(file_path).encode_wide().chain(Some(0)).collect();
    let open_as: Vec<u16> = OsStr::new("open").encode_wide().chain(Some(0)).collect();

    unsafe {
        ShellExecuteW(
            None,
            PWSTR(open_as.as_ptr() as *mut _),
            PWSTR(wide_path.as_ptr() as *mut _),
            PWSTR::null(),
            PWSTR::null(),
            SW_SHOW,
        );
    }
}

#[tauri::command]
pub fn open_file_with_selector(path: &str) {
    _open_file_with_selector(path);
}