use bindings::{
    windows::win32::windows_and_messaging::{
        HWND,
        GetMessageW,
        DispatchMessageW,
    },
    windows::win32::system_services::{
        DXGI_CREATE_FACTORY_DEBUG,
    },
    windows::win32::dxgi::{
        IDXGIFactory6,
        IDXGIFactory1,
    },
    windows::BOOL,
};

use std::ptr;
// use std::mem;

pub mod util;
pub mod win;
pub mod d3d;

const DEBUG: bool = true;
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn main() {

    let class_name = util::utf16_to_vec("D3D12Sample");

    let hwnd = win::create_window(&class_name, WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut dxgi_factory = ptr::null_mut();

    if DEBUG {
        dxgi_factory = d3d::create_dxgi_factory2::<IDXGIFactory6>(DXGI_CREATE_FACTORY_DEBUG).unwrap();
    } else {
        //dxgi_factory = lib::create_dxgi_factory1::<IDXGIFactory1>().unwrap();
    }

    let d3d12_device = d3d::create_d3d12_device().unwrap();

    win::show_window(hwnd);

    let mut msg = win::creat_message();

    while unsafe { GetMessageW(&mut msg, HWND(0), 0, 0).into() } {
        unsafe { DispatchMessageW(&mut msg); }

    }
}
