mod ui_interaction;
mod window_utils;

use ui_interaction::{click_button, click_lobby, type_text_and_enter};
use winapi::um::winuser::{GetWindowRect};
use winapi::shared::windef::RECT;
use window_utils::get_acr_poker_handle;
use std::{thread, time};

fn get_window_position(hwnd: winapi::shared::windef::HWND) -> Option<(i32, i32, i32, i32)> {
    unsafe {
        let mut rect: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        if GetWindowRect(hwnd, &mut rect) == 0 {
            return None;
        }
        Some((rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top))
    }
}

fn main() {
    if let Some(hwnd) = get_acr_poker_handle() {
        println!("ACR Poker window found: {:?}", hwnd);

        // Step 1: Click Tournament Tab
        click_button("TOURNAMENTS");
       
        // Step 2: Click Search Bar
        click_button("Search tournaments and players");

        // Step 3: Clear All Searches
        click_button("Clear All Searches");

        // Step 4: Click Search Bar Again
        click_button("Search tournaments and players");

        // Step 5: Type Tournament ID and Press Enter
        let tournament_id = "33206373";
        type_text_and_enter(tournament_id);

        // Step 6: Click BLITZ
        click_button("BLITZ");

        // Step 7: Click Tournament Tab
        click_button("TOURNAMENTS");
       
        // Step 8: Click Search Bar
        click_button("Search tournaments and players");
  
        // Step 9: Double Click Lobby Window
        click_lobby();

    } else {
        println!("ACR Poker window not found.");
    }
}
