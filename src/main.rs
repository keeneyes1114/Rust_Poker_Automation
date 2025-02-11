extern crate winapi;
use enigo::{Enigo, MouseButton, MouseControllable};
use winapi::um::winuser::{EnumWindows, GetWindowTextW, GetWindowRect, IsWindowVisible, keybd_event, VK_RETURN};
use winapi::shared::windef::RECT;
use winapi::shared::minwindef::{BOOL, LPARAM, BYTE};
use std::{ptr, thread, time};
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

fn get_acr_poker_handle() -> Option<winapi::shared::windef::HWND> {
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

fn get_window_position(hwnd: winapi::shared::windef::HWND) -> Option<(i32, i32, i32, i32)> {
    unsafe {
        let mut rect: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        if GetWindowRect(hwnd, &mut rect) == 0 {
            return None;
        }
        Some((rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top))
    }
}

fn click_at(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
    enigo.mouse_click(MouseButton::Left);
}

fn double_click_at(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
    enigo.mouse_click(MouseButton::Left);
    thread::sleep(time::Duration::from_millis(100));
    enigo.mouse_click(MouseButton::Left);
}

fn type_text_and_enter(text: &str) {
    for c in text.chars() {
        let vk = c as BYTE;
        unsafe {
            keybd_event(vk, 0, 0, 0); // Press key
            keybd_event(vk, 0, 2, 0); // Release key
        }
        thread::sleep(time::Duration::from_millis(50)); // Small delay between key presses
    }

    // Press Enter key to confirm search
    unsafe {
        keybd_event(VK_RETURN as BYTE, 0, 0, 0); // Press Enter
        keybd_event(VK_RETURN as BYTE, 0, 2, 0); // Release Enter
    }
    thread::sleep(time::Duration::from_secs(1)); // Allow UI to respond
}

fn main() {
    // let acr_poker_path = r"C:\ACR Poker\ACRPoker.exe";

    // match std::process::Command::new(acr_poker_path).spawn() {
    //     Ok(_) => println!("ACR Poker launched successfully!"),
    //     Err(e) => {
    //         eprintln!("Failed to launch ACR Poker: {}", e);
    //         return;
    //     }
    // }

    // thread::sleep(time::Duration::from_secs(20)); // Wait for the app to load

    if let Some(hwnd) = get_acr_poker_handle() {
        println!("ACR Poker window handle found: {:?}", hwnd);

        if let Some((x, y, width, height)) = get_window_position(hwnd) {
            println!("Window Position: ({}, {}), Size: {}x{}", x, y, width, height);

            let positions = [
                ("Tournament Tab", 500, 150),
                ("Search Bar", 1130, 195),
            ];

            // 1. Click Tournament Tab
            for (name, pos_x, pos_y) in &positions[..1] {
                let absolute_x = x + pos_x;
                let absolute_y = y + pos_y;
                println!("Clicking {} at ({}, {})", name, absolute_x, absolute_y);
                click_at(absolute_x, absolute_y);
                thread::sleep(time::Duration::from_secs(2)); // Wait after clicking
            }

            // 2. Click Search Bar
            let search_bar_x = x + 1130;
            let search_bar_y = y + 195;
            println!("Clicking Search Bar at ({}, {})", search_bar_x, search_bar_y);
            click_at(search_bar_x, search_bar_y);
            thread::sleep(time::Duration::from_secs(2)); // Wait after clicking

            // 3. Type Tournament ID and press Enter
            let tournament_id = "33204579";
            println!("Entering Tournament ID: {}", tournament_id);
            type_text_and_enter(tournament_id);
            thread::sleep(time::Duration::from_secs(2)); // Wait after entering the ID

            // 4. Double-click the first row
            let first_row_x = x + 480;
            let first_row_y = y + 305;
            println!("Double Clicking First Row at ({}, {})", first_row_x, first_row_y);
            double_click_at(first_row_x, first_row_y);

        } else {
            println!("Failed to get window position.");
        }
    } else {
        println!("ACR Poker window not found.");
    }

    println!("Successfully navigated to the tournament lobby!");
}
