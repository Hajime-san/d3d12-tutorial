use bindings::{
    windows::win32::windows_and_messaging as windows_and_messaging,
    windows::win32::system_services as system_services,
    windows::win32::dxgi as dxgi,
    windows::win32::direct3d12 as direct3d12,
    windows::BOOL,
};

use std::{ptr};
// use std::mem;

pub mod util;
pub mod win;
pub mod d3d;

const DEBUG: bool = true;
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn main() {

    let class_name = util::utf16_to_vec("D3D12Sample");

    let h_wnd = win::create_window(&class_name, WINDOW_WIDTH, WINDOW_HEIGHT);

    let dxgi_factory = d3d::create_dxgi_factory2::<dxgi::IDXGIFactory6>(system_services::DXGI_CREATE_FACTORY_DEBUG).unwrap();

    // dxgi_factory = d3d::create_dxgi_factory1::<IDXGIFactory1>().unwrap();

    /// enable debug layer
    d3d::enable_debug_layer(&DEBUG);

    /// create device
    let d3d12_device = d3d::create_d3d12_device().unwrap();

    /// create command list, allocator
    let command_allocator = d3d::create_command_allocator(&d3d12_device, direct3d12::D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT).unwrap();

    // create commnad queue
    let command_queue_desc = direct3d12::D3D12_COMMAND_QUEUE_DESC {
        flags : direct3d12::D3D12_COMMAND_QUEUE_FLAGS::D3D12_COMMAND_QUEUE_FLAG_NONE,
        node_mask : 0,
        priority : 0,
        r#type : direct3d12::D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT,
    };

    let command_queue = d3d::create_command_queue(&d3d12_device, &command_queue_desc).unwrap();

    // create swapchain
    let swapchain_desc1 = dxgi::DXGI_SWAP_CHAIN_DESC1 {
        width : WINDOW_WIDTH as u32,
        height : WINDOW_HEIGHT as u32,
        format : dxgi::DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM,
        stereo : BOOL(0),
        sample_desc: dxgi::DXGI_SAMPLE_DESC {
            count : 1,
            quality : 0,
        },
        buffer_usage : dxgi::DXGI_USAGE_BACK_BUFFER,
        buffer_count : 2,
        scaling : dxgi::DXGI_SCALING::DXGI_SCALING_STRETCH,
        swap_effect : dxgi::DXGI_SWAP_EFFECT::DXGI_SWAP_EFFECT_FLIP_DISCARD,
        alpha_mode : dxgi::DXGI_ALPHA_MODE::DXGI_ALPHA_MODE_UNSPECIFIED,
        flags : 0,
    };

    let swapchain = d3d::create_swap_chain_for_hwnd(&dxgi_factory, &command_queue, h_wnd, &swapchain_desc1, ptr::null_mut(), None);

    // let command_list = d3d::create_command_list(d3d12_device, 0, D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT, command_allocator, ptr::null_mut()).unwrap();

    win::show_window(h_wnd);

    let mut message = win::creat_message();

    while unsafe { windows_and_messaging::GetMessageW(&mut message, windows_and_messaging::HWND(0), 0, 0).into() } {
        unsafe { windows_and_messaging::DispatchMessageW(&mut message); }

    }
}
