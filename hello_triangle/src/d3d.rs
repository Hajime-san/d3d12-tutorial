use bindings::{
    windows::win32::com::{
        HRESULT,
    },
    windows::win32::system_services::{
        S_FALSE,
    },
    windows::win32::dxgi::{
        CreateDXGIFactory1,
        CreateDXGIFactory2,
    },
    windows::win32::direct3d12::{
        ID3D12Device,
        D3D12CreateDevice,
    },
    windows::Interface,
    windows::IUnknown,
    windows::ErrorCode,
    windows::win32::direct3d11::D3D_FEATURE_LEVEL,
};

use std::ptr;

use crate::util;

pub fn create_dxgi_factory1<T: Interface>() -> Result<*mut T, ErrorCode> {

    let mut obj = ptr::null_mut::<T>();

    let result = unsafe {
        CreateDXGIFactory1(&T::IID, util::get_pointer_of_interface(&mut obj))
    };

    match result {
        S_OK => Ok(obj),
        _ => Err(result)
    }
}

pub fn create_dxgi_factory2<T: Interface>(flags: u32) -> Result<*mut T, ErrorCode> {

    let mut obj = ptr::null_mut::<T>();

    let result = unsafe {
        CreateDXGIFactory2(flags, &T::IID, util::get_pointer_of_interface(&mut obj))
    };

    match result {
        S_OK => Ok(obj),
        _ => Err(result)
    }
}

pub fn create_d3d12_device() -> Result<*mut ID3D12Device, ErrorCode> {

    let levels: [D3D_FEATURE_LEVEL; 4] = [
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_12_1,
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_12_0,
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_11_1,
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_11_0
    ];

    let mut obj = ptr::null_mut::<ID3D12Device>();

    for lv in levels.iter() {

        if unsafe {
                D3D12CreateDevice(
                    None,
                    *lv,
                    &ID3D12Device::IID,
                    util::get_pointer_of_interface(&mut obj)
                )
                == ErrorCode::S_OK
            }
            {
                break;
        }
    }

    match obj.is_null() {
        true => Err(ErrorCode::E_POINTER),
        _ => Ok(obj)
    }
}
