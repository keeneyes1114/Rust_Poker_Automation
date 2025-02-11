mod ui_interaction;
mod window_utils;

use ui_interaction::{click_button, click_lobby_pos, type_text_and_enter, is_existed};
use winapi::um::winuser::{GetWindowRect};
use winapi::shared::windef::RECT;
use window_utils::get_acr_poker_handle;
use std::{thread, time};

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
    if let Some(hwnd) = get_acr_poker_handle() {
        println!("ACR Poker window found: {:?}", hwnd);

        // Step 1: Click Tournament Tab
        click_button("TOURNAMENTS");
        thread::sleep(time::Duration::from_millis(6000));
       
        // Step 2: Click Search Bar
        click_button("Search tournaments and players");

        // Step 3: Type Tournament ID and Press Enter
        let tournament_id = "33206373";
        type_text_and_enter(tournament_id);

        // Step 4: Double Click First Row
        if let Some((x, y)) = get_window_position(hwnd) {
            let first_row_x = x + 480;
            let first_row_y = y + 305;
            println!("Double Clicking First Row at ({}, {})", first_row_x, first_row_y);
            click_lobby_pos(first_row_x, first_row_y);
        }

        // Step 5: Check "No games match"
        let exists = is_existed("No games match your filter criteria. Change your filters to see games."); // Call the function and get the result
        println!("exists: {}", exists);
        if exists {
            // Step 1: Click Daily Schedule
            click_button("DAILY SCHEDULE");
            
            // Step 2: Click Search Bar
            click_button("Search tournaments and players");

            // Step 3: Type Tournament ID and Press Enter
            let tournament_id = "33206373";
            type_text_and_enter(tournament_id);

            // Step 4: Double Click First Row
            if let Some((x, y)) = get_window_position(hwnd) {
                let first_row_x = x + 480;
                let first_row_y = y + 305;
                println!("Double Clicking First Row at ({}, {})", first_row_x, first_row_y);
                click_lobby_pos(first_row_x, first_row_y);
            }
        }

    } else {
        println!("ACR Poker window not found.");
    }
}