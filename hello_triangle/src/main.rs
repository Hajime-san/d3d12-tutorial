use bindings::{
    windows::win32::windows_and_messaging::{
        HWND,
        GetMessageW,
        DispatchMessageW,
    },
    windows::BOOL,
};

// use std::mem;

pub mod util;
pub mod win;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn main() {

    let class_name = util::utf16_to_vec("D3D12Sample");

    let hwnd = win::create_window(&class_name, WINDOW_WIDTH, WINDOW_HEIGHT);

    win::show_window(hwnd);

    let mut msg = win::creat_message();

    while unsafe { GetMessageW(&mut msg, HWND(0), 0, 0).into() } {
        unsafe { DispatchMessageW(&mut msg); }

    }
}
