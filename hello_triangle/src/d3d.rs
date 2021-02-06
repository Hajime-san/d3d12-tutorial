use bindings::{
    windows::win32::dxgi::{
        CreateDXGIFactory1,
        CreateDXGIFactory2,
    },
    windows::win32::direct3d12::{
        ID3D12Device,
        D3D12CreateDevice,
        ID3D12Debug,
        D3D12GetDebugInterface,
        ID3D12CommandAllocator,
        ID3D12GraphicsCommandList,
        ID3D12PipelineState,
        D3D12_COMMAND_LIST_TYPE,
    },
    windows::Interface,
    windows::IUnknown,
    windows::ErrorCode,
    windows::win32::direct3d11::D3D_FEATURE_LEVEL,
    windows::Abi,
    windows::Result as WinResult,
};

use std::ptr;

use crate::util;

pub fn create_dxgi_factory1<T:Interface>() -> WinResult<T> {
    unsafe {
        let mut dxfactory: Option<T> = None;
        CreateDXGIFactory1(&T::IID, dxfactory.set_abi()).and_some(dxfactory)
    }
}

pub fn create_dxgi_factory2<T:Interface>(flags: u32) -> WinResult<T> {
    unsafe {
        let mut dxfactory: Option<T> = None;
        CreateDXGIFactory2(
            flags,
            &T::IID,
            dxfactory.set_abi()
        )
        .and_some(dxfactory)
    }
}

pub fn create_d3d12_device() -> WinResult<ID3D12Device> {

    let levels: [D3D_FEATURE_LEVEL; 4] = [
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_12_1,
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_12_0,
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_11_1,
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_11_0
    ];

    let mut result: ErrorCode = ErrorCode::E_POINTER;
    let mut d3d_device: Option<ID3D12Device> = None;

    for lv in levels.iter() {

        unsafe {
            result = D3D12CreateDevice(
                None,
                *lv,
                &ID3D12Device::IID,
                d3d_device.set_abi()
            )
        };

        if result == ErrorCode::S_OK {
            break;
        }
    }

    result.and_some(d3d_device)
}

pub fn enable_debug_layer(is_debug: bool) {
    if !is_debug {
        return;
    }

    unsafe {
        let mut debug_interface: Option<ID3D12Debug> = None;
        D3D12GetDebugInterface(
            &ID3D12Debug::IID,
            debug_interface.set_abi()
        ).and_some(debug_interface)
        .as_ref().unwrap()
        .EnableDebugLayer();
    }
}

pub fn create_command_allocator(device: *const ID3D12Device, type_: D3D12_COMMAND_LIST_TYPE) -> Result<*const ID3D12CommandAllocator, ErrorCode> {

    let mut obj = ptr::null_mut::<ID3D12CommandAllocator>();

    let result = unsafe {
        device.as_ref().unwrap().
            CreateCommandAllocator(
                type_,
                &ID3D12CommandAllocator::IID,
                util::get_pointer_of_interface(&mut obj)
            )
    };

    match result {
        S_OK => Ok(obj),
        _ => Err(result)
    }
}

// pub fn create_command_list(device: *const ID3D12Device, nodeMask: u32, type_: D3D12_COMMAND_LIST_TYPE, pCommandAllocator: *const ID3D12CommandAllocator, pInitialState: *mut ID3D12PipelineState) -> Result<*const ID3D12GraphicsCommandList, ErrorCode> {

//     let mut obj = ptr::null_mut::<ID3D12GraphicsCommandList>();

//     let result = unsafe {
//         device.as_ref().unwrap().
//             CreateCommandList(
//                 nodeMask,
//                 type_,
//                 pCommandAllocator,
//                 pInitialState,
//                 &ID3D12GraphicsCommandList::IID,
//                 util::get_pointer_of_interface(&mut obj)
//             )
//     };

//     match result {
//         S_OK => Ok(obj),
//         _ => Err(result)
//     }
// }
