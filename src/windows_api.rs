use windows::{
    Win32::UI::WindowsAndMessaging::{
        GetWindowLongW,
        SetWindowLongW,
        GWL_EXSTYLE,
        WS_EX_LAYERED,
        WS_EX_TRANSPARENT,
        GetWindowRect,
        FindWindowA
    },
    Win32::Foundation:: { HWND, RECT },
};
use winit::raw_window_handle::{ Win32WindowHandle, RawWindowHandle };
use winit::platform;
use windows_core::PCSTR;
use std::ffi::CString;

pub fn grab_handle(window_handle: winit::raw_window_handle::WindowHandle) -> Option<platform::windows::HWND> {
    let raw_handle = window_handle.as_raw();

    let win32_handle: Win32WindowHandle = if let RawWindowHandle::Win32(hwnd) = raw_handle {
        hwnd
    } else {
        return None
    };

    let handle = win32_handle.hwnd.into();

    Some(handle)
}

pub fn grab_game_hwnd(input: &str) -> HWND {
    let window_name = CString::new(input).unwrap();

    let hwnd = unsafe { FindWindowA(None, PCSTR::from_raw(window_name.as_ptr() as _))
                        .expect("Process Not Open!") };
    hwnd
}

pub fn make_window_click_through(hwnd: HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32);
    }
}

pub fn make_window_non_click_through(hwnd: HWND) {
    unsafe {
        let mut ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        ex_style &= !(WS_EX_LAYERED.0 as i32);
        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_TRANSPARENT.0 as i32);
    }
}

pub fn grab_window_dimensions(hwnd: HWND) -> RECT {
    unsafe {
        let mut window_dimensions: RECT = std::mem::zeroed();
        _ = GetWindowRect(hwnd, &mut window_dimensions);
        return window_dimensions
    }
}
