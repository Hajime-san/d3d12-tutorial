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
    windows::win32::direct3d12::{
        D3D12_COMMAND_LIST_TYPE,
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

    let dxgi_factory = d3d::create_dxgi_factory2::<IDXGIFactory6>(DXGI_CREATE_FACTORY_DEBUG).unwrap();

    // dxgi_factory = d3d::create_dxgi_factory1::<IDXGIFactory1>().unwrap();

    /// enable debug layer
    d3d::enable_debug_layer(DEBUG);

    /// create device
    let d3d12_device = d3d::create_d3d12_device().unwrap();

    /// create command list, allocator
    // let command_allocator = d3d::create_command_allocator(d3d12_device, D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT).unwrap();
    // let command_list = d3d::create_command_list(d3d12_device, 0, D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT, command_allocator, ptr::null_mut()).unwrap();

    win::show_window(hwnd);

    let mut msg = win::creat_message();

    while unsafe { GetMessageW(&mut msg, HWND(0), 0, 0).into() } {
        unsafe { DispatchMessageW(&mut msg); }

    }
}
