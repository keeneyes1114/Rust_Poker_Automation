mod ui_interaction;
mod window_utils;
mod color;

use ui_interaction::{click_button, click_lobby_pos, type_text_and_enter};
use winapi::um::winuser::{GetWindowRect};
use winapi::shared::windef::RECT;
use window_utils::get_acr_poker_handle;
use color::get_average_color;

fn get_window_position(hwnd: winapi::shared::windef::HWND) -> Option<(i32, i32)> {
    unsafe {
        let mut rect: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        if GetWindowRect(hwnd, &mut rect) == 0 {
            return None;
        }
        Some((rect.left, rect.top))
    }
}

fn main() {
    match get_average_color(100, 100) {
        Some(avg_color) => println!("Average color: {:?}", avg_color),
        None => println!("Could not get the color"),
    }

    if let Some(hwnd) = get_acr_poker_handle() {
        println!("ACR Poker window found: {:?}", hwnd);

        // Step 1: Click Tournament Tab
        click_button("TOURNAMENTS");
        // thread::sleep(time::Duration::from_millis(2000));

        // Step 2: Check Daily Schedule, If not active, click Daily Schedule button
        if let Some((x, y)) = get_window_position(hwnd) {
            let first_row_x = x + 80;
            let first_row_y = y + 193;
            match get_average_color(first_row_x, first_row_y) {
                Some(avg_color) => {
                    println!("Average color: {:?}", avg_color);
                    if avg_color[0] < 90 { // Assuming avg_color is a tuple (R, G, B)
                        click_button("DAILY SCHEDULE");
                    }
                }
                None => println!("Could not get the color"),
            }
        }
        
        // Step 3: Click Search Bar
        click_button("Search tournaments and players");

        // Step 4: Type Tournament ID and Press Enter
        let tournament_id = "33209888";
        type_text_and_enter(tournament_id);

        // Step 5: Double Click First Row
        if let Some((x, y)) = get_window_position(hwnd) {
            let first_row_x = x + 480;
            let first_row_y = y + 305;
            println!("Double Clicking First Row at ({}, {})", first_row_x, first_row_y);
            click_lobby_pos(first_row_x, first_row_y);
        }

    } else {
        println!("ACR Poker window not found.");
    }
}