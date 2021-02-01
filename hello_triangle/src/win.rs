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
    windows::win32::windows_and_messaging::{
        WNDCLASSW,
        HWND,
        LPARAM,
        WPARAM,
        MSG,
        RegisterClassW,
        CreateWindowExW,
        PostQuitMessage,
        DefWindowProcW,
        ShowWindow,
        GetMessageW,
        DispatchMessageW,
    },
    windows::win32::gdi::{
        HBRUSH,
        GetStockObject,
        UpdateWindow,
    },
    windows::win32::menus_and_resources::{
        LoadIconW,
        LoadCursorW,
        HMENU,
    },
    windows::win32::windows_programming::{
        CloseHandle,
    },
    windows::BOOL,
};

use std::ptr;
// use std::mem;


pub fn create_window(class_name: &[u16], width: i32, height: i32) -> HWND {

    let instance = unsafe { HINSTANCE(GetModuleHandleW(std::ptr::null())) };

    let mut winc = WNDCLASSW::default();
    winc.style = (CS_HREDRAW | CS_VREDRAW) as u32;
    winc.lpfn_wnd_proc = Some(window_procedure);
    winc.h_icon = unsafe { LoadIconW(instance, IDI_APPLICATION as *const u16) };
    winc.h_cursor = unsafe { LoadCursorW(instance, IDC_ARROW as *const u16) };
    // winc.hbr_background = unsafe { GetStockObject(WHITE_BRUSH as i32) as HBRUSH };
    winc.lpsz_class_name = class_name.as_ptr() as *mut u16;

    let atom = unsafe { RegisterClassW(&winc) };
    debug_assert!(atom != 0);

    unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            class_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            0,
            0,
            width,
            height,
            HWND(0),
            HMENU(0),
            instance,
            ptr::null_mut(),
        )
    }

}

pub extern "system" fn window_procedure(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {

    match msg as i32 {
        WM_DESTROY => unsafe {
                        PostQuitMessage(0);
                        LRESULT(0)
                    },
        _ => return unsafe {
                        DefWindowProcW(hwnd, msg, w_param, l_param)
                    },
    }

}

pub fn creat_message() -> MSG {
    MSG::default()
}

pub fn show_window(hwnd: HWND) {
    unsafe {
        ShowWindow(hwnd, SW_NORMAL);
        UpdateWindow(hwnd);
    }
}
