extern crate winapi;
use winapi::um::winuser::{EnumWindows, GetWindowTextW, IsWindowVisible};
use winapi::shared::minwindef::{BOOL, LPARAM};
use std::{ptr};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

unsafe extern "system" fn enum_windows_proc(hwnd: winapi::shared::windef::HWND, lparam: LPARAM) -> BOOL {
    let mut buffer: [u16; 256] = [0; 256];
    let length = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);

    if length > 0 && IsWindowVisible(hwnd) != 0 {
        let title = OsString::from_wide(&buffer[..length as usize])
            .to_string_lossy()
            .to_string();

        println!("Found Window: {}", title);

        if title.contains("ACR Poker") {
            let handle_ptr = lparam as *mut winapi::shared::windef::HWND;
            *handle_ptr = hwnd;
            return 0; // Stop enumeration
        }
    }
    1
}

pub fn get_acr_poker_handle() -> Option<winapi::shared::windef::HWND> {
    let mut hwnd: winapi::shared::windef::HWND = ptr::null_mut();
    unsafe {
        EnumWindows(Some(enum_windows_proc), &mut hwnd as *mut _ as LPARAM);
    }
    if hwnd.is_null() {
        None
    } else {
        Some(hwnd)
    }
}
