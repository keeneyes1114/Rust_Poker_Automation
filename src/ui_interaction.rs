
extern crate winapi;
use winapi::um::winuser::{keybd_event, VK_RETURN, VK_BACK, VK_CONTROL};
use winapi::shared::minwindef::{BYTE};
use std::{thread, time};

use enigo::{Enigo, MouseButton, MouseControllable};
use uiautomation::UIAutomation;

pub fn click_button(button_name: &str) {
    let automation = UIAutomation::new().unwrap();
    
    let matcher = automation.create_matcher().name(button_name);
    if let Ok(button) = matcher.find_first() {
        button.set_focus().unwrap(); // Ensure the button is focused before clicking
        
        if let Err(e) = button.click() {
            eprintln!("Failed to click the button '{}': {:?}", button_name, e);
        }
    } else {
        println!("Button '{}' not found.", button_name);
    }
}

pub fn type_text_and_enter(text: &str) {
    unsafe {
        // Press "Ctrl + A" to select all text
        keybd_event(VK_CONTROL as BYTE, 0, 0, 0); // Press Ctrl
        keybd_event(0x41 as BYTE, 0, 0, 0); // Press A
        keybd_event(0x41 as BYTE, 0, 2, 0); // Release A
        keybd_event(VK_CONTROL as BYTE, 0, 2, 0); // Release Ctrl

        thread::sleep(time::Duration::from_millis(10)); // Small delay

        // Press "Backspace" to delete the selected text
        keybd_event(VK_BACK as BYTE, 0, 0, 0); // Press Backspace
        keybd_event(VK_BACK as BYTE, 0, 2, 0); // Release Backspace

        thread::sleep(time::Duration::from_millis(10)); // Small delay
    }

    for c in text.chars() {
        let vk = c as BYTE;
        unsafe {
            keybd_event(vk, 0, 0, 0); // Press key
            keybd_event(vk, 0, 2, 0); // Release key
        }
        thread::sleep(time::Duration::from_millis(1)); // Small delay between key presses
    }

    // Press Enter key to confirm search
    unsafe {
        keybd_event(VK_RETURN as BYTE, 0, 0, 0); // Press Enter
        keybd_event(VK_RETURN as BYTE, 0, 2, 0); // Release Enter
    }
    thread::sleep(time::Duration::from_secs(1)); // Allow UI to respond
}

// pub fn click_lobby() {
//     unsafe {
//         // Press "Tab" 8 times
//         for _ in 0..8 {
//             keybd_event(VK_TAB as u8, 0, 0, 0); // Press Tab
//             keybd_event(VK_TAB as u8, 0, 2, 0); // Release Tab
//             thread::sleep(time::Duration::from_millis(100)); // Small delay for UI responsiveness
//         }

//         // Press "Enter"
//         keybd_event(VK_RETURN as u8, 0, 0, 0); // Press Enter
//         keybd_event(VK_RETURN as u8, 0, 2, 0); // Release Enter
//     }

//     thread::sleep(time::Duration::from_secs(1)); // Allow UI to respond
// }

pub fn click_lobby_pos(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
    enigo.mouse_click(MouseButton::Left);
    thread::sleep(time::Duration::from_millis(100));
    enigo.mouse_click(MouseButton::Left);
}
