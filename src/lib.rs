#![allow(non_snake_case)]

extern crate winapi;

use winapi::um::processthreadsapi::{CreateThread};
use winapi::um::handleapi::CloseHandle;
use winapi::shared::minwindef::LPVOID;
use winapi::um::libloaderapi::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
use winapi::um::wincon::AttachConsole;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::consoleapi::AllocConsole;
use winapi::um::processenv::GetStdHandle;
use std::ptr::null_mut;
use std::thread;
use std::ffi::{c_void};
use std::time::Duration;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

unsafe extern "system" fn setup(lp_param: LPVOID) -> u32 {
    /*core::memory::SETUP();
    core::interfaces::SETUP();
    core::netvars::SETUP();
    core::hooks::SETUP();*/

    while winapi::um::winuser::GetAsyncKeyState(winapi::um::winuser::VK_END) == 0 {
        println!("no insert");
        thread::sleep(Duration::from_millis(200));
    }

    println!("insert");
    //core::hooks::destroy();
    FreeLibraryAndExitThread(lp_param as _, 0);
    0
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(hinst_dll: *mut c_void, fdw_reason: u32, _: *mut c_void) -> i32 {
    if fdw_reason == DLL_PROCESS_ATTACH {
        DisableThreadLibraryCalls(hinst_dll as _);

        if AttachConsole(winapi::um::wincon::ATTACH_PARENT_PROCESS) == 0 {
            let std_output_handle = GetStdHandle(STD_OUTPUT_HANDLE);
            if std_output_handle.is_null() {
                AllocConsole();
            }
        }

        println!("Hello from injected DLL!");

        let h_thread = CreateThread(
            null_mut(),
            0,
            Some(setup),
            hinst_dll as LPVOID,
            0,
            null_mut(),
        );

        if !h_thread.is_null() {
            CloseHandle(h_thread);
        }
    }

    1
}