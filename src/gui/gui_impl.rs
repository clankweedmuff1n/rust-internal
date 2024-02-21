extern crate winapi;

use winapi::um::winuser::{HWND, WNDPROC};
use winapi::um::libloaderapi::GetModuleHandleW;

use crate::gui::gui;

extern "system" fn window_process(window: HWND, message: UINT, wide_param: WPARAM, long_param: LPARAM) -> LRESULT {
    // Your window procedure implementation
    unimplemented!()
}

pub unsafe extern "system" fn gui_window_process(window: HWND, message: UINT, wide_param: WPARAM, long_param: LPARAM) -> LRESULT {
    if GetAsyncKeyState(VK_INSERT) & 1 {
        gui::open = !gui::open;
    }

    if gui::open && ImGui_ImplWin32_WndProcHandler(
        window,
        message,
        wide_param,
        long_param,
    ) {
        return 1;
    }

    CallWindowProcW(
        gui::originalWindowProcess,
        window,
        message,
        wide_param,
        long_param,
    )
}
