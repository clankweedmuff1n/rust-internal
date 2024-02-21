#![allow(non_snake_case)]

extern crate winapi;

use winapi::um::processthreadsapi::{CreateThread};
use winapi::um::handleapi::CloseHandle;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::libloaderapi::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
use std::ptr::null_mut;
use std::{ptr, thread};
use std::ffi::c_void;
use std::time::Duration;
use winapi::um::minwinbase::LPTHREAD_START_ROUTINE;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

unsafe extern "system" fn setup(lp_param: LPVOID) -> u32 {
    /*core::memory::setup();
    core::interfaces::setup();
    core::netvars::setup();
    core::hooks::setup();*/

    // Sleep the thread until the unload key is pressed
    while winapi::um::winuser::GetAsyncKeyState(winapi::um::winuser::VK_END) == 0 {
        thread::sleep(Duration::from_millis(200));
    }

    //core::hooks::destroy();
    FreeLibraryAndExitThread(lp_param as _, 0);
    0
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(hinst_dll: *mut c_void, fdw_reason: u32, _: *mut c_void) -> i32 {
    if fdw_reason == DLL_PROCESS_ATTACH {
        DisableThreadLibraryCalls(hinst_dll as _);

        // Create the setup thread
        let h_thread = CreateThread(
            ptr::null_mut(),
            0,
            Some(setup),
            hinst_dll as LPVOID,
            0,
            ptr::null_mut(),
        );

        if !h_thread.is_null() {
            winapi::um::handleapi::CloseHandle(h_thread);
        }
    }

    // Successful DLL_PROCESS_ATTACH
    1
}