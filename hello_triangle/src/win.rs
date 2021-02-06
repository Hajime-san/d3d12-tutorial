use bindings::{
    windows::win32::system_services::{
        CreateEventW,
        GetModuleHandleW,
        SetEvent,
        WaitForSingleObject,
        CS_HREDRAW,
        CS_VREDRAW,
        IDI_APPLICATION,
        IDC_ARROW,
        WS_OVERLAPPEDWINDOW,
        WM_DESTROY,
        WM_QUIT,
        LRESULT,
        SW_NORMAL,
        HINSTANCE,
    },
    windows::win32::windows_and_messaging as windows_and_messaging,
    windows::win32::gdi as gdi,
    windows::win32::menus_and_resources as menus_and_resources,
    windows::win32::windows_programming as windows_programming,
    windows::BOOL,
};

use std::ptr;

pub fn create_window(class_name: &[u16], width: i32, height: i32) -> windows_and_messaging::HWND {

    let instance = unsafe { HINSTANCE(GetModuleHandleW(std::ptr::null())) };

    let mut window_class = windows_and_messaging::WNDCLASSW::default();
    window_class.style = (CS_HREDRAW | CS_VREDRAW) as u32;
    window_class.lpfn_wnd_proc = Some(window_procedure);
    window_class.h_icon = unsafe { menus_and_resources::LoadIconW(instance, IDI_APPLICATION as *const u16) };
    window_class.h_cursor = unsafe { menus_and_resources::LoadCursorW(instance, IDC_ARROW as *const u16) };
    // window_class.hbr_background = unsafe { GetStockObject(WHITE_BRUSH as i32) as HBRUSH };
    window_class.lpsz_class_name = class_name.as_ptr() as *mut u16;

    let atom = unsafe { windows_and_messaging::RegisterClassW(&window_class) };
    debug_assert!(atom != 0);

    unsafe {
        windows_and_messaging::CreateWindowExW(
            0,
            class_name.as_ptr(),
            class_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            0,
            0,
            width,
            height,
            windows_and_messaging::HWND(0),
            menus_and_resources::HMENU(0),
            instance,
            ptr::null_mut(),
        )
    }
}

pub extern "system" fn window_procedure(
    h_wnd: windows_and_messaging::HWND,
    message: u32,
    w_param: windows_and_messaging::WPARAM,
    l_param: windows_and_messaging::LPARAM)
    -> LRESULT {
    match message as i32 {
        WM_DESTROY =>
        unsafe {
            windows_and_messaging::PostQuitMessage(0);
            LRESULT(0)
        },
        _ =>
        unsafe {
            windows_and_messaging::DefWindowProcW(h_wnd, message, w_param, l_param)
        },
    }
}

pub fn creat_message() -> windows_and_messaging::MSG {
    windows_and_messaging::MSG::default()
}

pub fn show_window(h_wnd: windows_and_messaging::HWND) {
    unsafe {
        windows_and_messaging::ShowWindow(h_wnd, SW_NORMAL);
        gdi::UpdateWindow(h_wnd);
    }
}
