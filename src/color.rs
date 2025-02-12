extern crate winapi;
extern crate image;

use winapi::um::winuser::{GetDC};
use winapi::um::wingdi::{GetPixel};
use winapi::shared::windef::{HWND};
use std::ptr;
use image::{Rgb};

fn get_pixel_color(hwnd: HWND, x: i32, y: i32) -> Option<Rgb<u8>> {
    unsafe {
        // Get the device context for the window
        let hdc = GetDC(hwnd);
        if hdc.is_null() {
            return None;
        }

        // Get the color of the pixel at (x, y)
        let pixel_color = GetPixel(hdc, x, y);
        if pixel_color == 0xFFFFFFFF {
            return None;
        }

        // Extract RGB values from the pixel color
        let red = (pixel_color & 0xFF) as u8;
        let green = ((pixel_color >> 8) & 0xFF) as u8;
        let blue = ((pixel_color >> 16) & 0xFF) as u8;

        // Release the device context
        winapi::um::winuser::ReleaseDC(hwnd, hdc);

        Some(Rgb([red, green, blue]))
    }
}

pub fn get_average_color(x: i32, y: i32) -> Option<Rgb<u8>> {
    let hwnd: HWND = ptr::null_mut(); // Replace with your window handle
    let mut sum_red = 0u32;
    let mut sum_green = 0u32;
    let mut sum_blue = 0u32;
    let mut count = 0u32;

    // Capture a 20x20 pixel area around (x, y)
    for dx in -5..=5 {
        for dy in -5..=5 {
            if let Some(color) = get_pixel_color(hwnd, x + dx, y + dy) {
                sum_red += color[0] as u32;
                sum_green += color[1] as u32;
                sum_blue += color[2] as u32;
                count += 1;
            }
        }
    }

    if count > 0 {
        // Calculate average
        let avg_red = (sum_red / count) as u8;
        let avg_green = (sum_green / count) as u8;
        let avg_blue = (sum_blue / count) as u8;

        Some(Rgb([avg_red, avg_green, avg_blue]))
    } else {
        None
    }
}