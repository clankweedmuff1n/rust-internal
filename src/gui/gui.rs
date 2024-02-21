#![allow(non_snake_case)]

use std::ffi::CString;
use std::ptr::null_mut;
use winapi::um::d3d9::{D3DCREATE_DISABLE_DRIVER_MANAGEMENT, D3DCREATE_SOFTWARE_VERTEXPROCESSING, Direct3DCreate9, D3DPRESENT_PARAMETERS, D3D_SDK_VERSION};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::winuser::{CreateWindowExA, DefWindowProcA, RegisterClassExA, UnregisterClassA, HWND, WNDCLASSEXA, WNDPROC, WS_OVERLAPPEDWINDOW, CS_HREDRAW, CS_VREDRAW, GWLP_WNDPROC, SetWindowLongPtrA, CallWindowProcA, VK_INSERT};
use winapi::Interface;

pub mod gui {
    // Show menu
    pub static mut OPEN: bool = true;

    // Is menu SETUP
    pub static mut SETUP: bool = false;

    // WinAPI related
    pub static mut WINDOW: HWND = null_mut();
    pub static mut WINDOW_CLASS: WNDCLASSEXA = WNDCLASSEXA::default();
    pub static mut ORIGINAL_WINDOW_PROCESS: WNDPROC = null_mut();

    // DirectX
    pub static mut DEVICE: *mut IDirect3DDevice9 = null_mut();
    pub static mut D3D9: *mut IDirect3D9 = null_mut();

    pub fn setupWindowClass(window_class_name: &str) -> bool {
        unsafe {
            WINDOW_CLASS.cbSize = std::mem::size_of::<WNDCLASSEXA>() as u32;
            WINDOW_CLASS.style = CS_HREDRAW | CS_VREDRAW;
            WINDOW_CLASS.lpfnWndProc = Some(DefWindowProcA);
            WINDOW_CLASS.cbClsExtra = 0;
            WINDOW_CLASS.cbWndExtra = 0;
            WINDOW_CLASS.hInstance = GetModuleHandleA(null_mut());
            WINDOW_CLASS.hIcon = null_mut();
            WINDOW_CLASS.hCursor = null_mut();
            WINDOW_CLASS.hbrBackground = null_mut();
            WINDOW_CLASS.lpszMenuName = null_mut();
            WINDOW_CLASS.lpszClassName = CString::new(window_class_name).unwrap().as_ptr();
            WINDOW_CLASS.hIconSm = null_mut();

            if RegisterClassExA(&WINDOW_CLASS) == 0 {
                return false;
            }
        }
        true
    }

    pub fn destroyWindowClass() {
        unsafe {
            UnregisterClassA(
                WINDOW_CLASS.lpszClassName,
                WINDOW_CLASS.hInstance,
            );
        }
    }

    pub fn setupWindow(window_name: &str) -> bool {
        unsafe {
            WINDOW = CreateWindowExA(
                0,
                WINDOW_CLASS.lpszClassName,
                CString::new(window_name).unwrap().as_ptr(),
                WS_OVERLAPPEDWINDOW,
                0,
                0,
                100,
                100,
                null_mut(),
                null_mut(),
                WINDOW_CLASS.hInstance,
                null_mut(),
            );

            if WINDOW.is_null() {
                return false;
            }
        }
        true
    }

    pub fn destroyWindow() {
        unsafe {
            if !WINDOW.is_null() {
                DestroyWindow(WINDOW);
            }
        }
    }

    pub fn setupDirectX() -> bool {
        unsafe {
            let handle = GetModuleHandleA(CString::new("D3D9.dll").unwrap().as_ptr());
            if handle.is_null() {
                return false;
            }

            let create = GetProcAddress(handle, CString::new("Direct3DCreate9").unwrap().as_ptr());
            if create.is_null() {
                return false;
            }

            let create_fn: extern "system" fn(u32) -> *mut IDirect3D9 = std::mem::transmute(create);
            D3D9 = create_fn(D3D_SDK_VERSION);

            if D3D9.is_null() {
                return false;
            }

            let mut params: D3DPRESENT_PARAMETERS = Default::default();
            params.Windowed = 1;
            params.hDeviceWindow = WINDOW;
            params.SwapEffect = D3DSWAPEFFECT_DISCARD;

            if (*D3D9).CreateDevice(
                D3DADAPTER_DEFAULT,
                D3DDEVTYPE_NULLREF,
                WINDOW,
                D3DCREATE_SOFTWARE_VERTEXPROCESSING | D3DCREATE_DISABLE_DRIVER_MANAGEMENT,
                &mut params,
                &mut DEVICE,
            ) < 0 {
                return false;
            }
        }
        true
    }

    pub fn destroyDirectX() {
        unsafe {
            if !DEVICE.is_null() {
                (*DEVICE).Release();
                DEVICE = null_mut();
            }

            if !D3D9.is_null() {
                (*D3D9).Release();
                D3D9 = null_mut();
            }
        }
    }

    pub fn Setup() {
        unsafe {
            if !setupWindowClass("hackClass001") {
                panic!("Failed to create window class.");
            }

            if !setupWindow("Hack Window") {
                panic!("Failed to create window.");
            }

            if !setupDirectX() {
                panic!("Failed to create DirectX.");
            }

            destroyWindow();
            destroyWindowClass();
        }
    }

    pub fn setupMenu(DEVICE: *mut IDirect3DDevice9) {
        unsafe {
            let mut params = D3DDEVICE_CREATION_PARAMETERS::default();
            (*DEVICE).GetCreationParameters(&mut params);

            WINDOW = params.hFocusWindow;

            ORIGINAL_WINDOW_PROCESS = SetWindowLongPtrA(
                WINDOW,
                GWLP_WNDPROC,
                WindowProcess as isize,
            ) as WNDPROC;

            ImGui::CreateContext();
            ImGui::StyleColorsDark();

            ImGui_ImplWin32_Init(WINDOW);
            ImGui_ImplDX9_Init(DEVICE);

            SETUP = true;
        }
    }

    pub fn destroy() {
        unsafe {
            ImGui_ImplWin32_Shutdown();
            ImGui_ImplDX9_Shutdown();
            ImGui::DestroyContext();

            SetWindowLongPtrA(
                WINDOW,
                GWLP_WNDPROC,
                ORIGINAL_WINDOW_PROCESS as isize,
            );

            destroyDirectX();
        }
    }

    pub fn render() {
        unsafe {
            ImGui_ImplDX9_NewFrame();
            ImGui_ImplWin32_NewFrame();
            ImGui::NewFrame();

            ImGui::Begin("menu", &mut OPEN);
            ImGui::End();

            ImGui::EndFrame();
            ImGui::Render();
            ImGui_ImplDX9_RenderDrawData(ImGui::GetDrawData());
        }
    }

    pub unsafe extern "system" fn windowProcess(
        WINDOW: HWND,
        message: u32,
        wideParam: WPARAM,
        longParam: LPARAM,
    ) -> isize {
        if GetAsyncKeyState(VK_INSERT) & 1 != 0 {
            OPEN = !OPEN;
        }

        if OPEN && ImGui_ImplWin32_WndProcHandler(
            WINDOW,
            message,
            wideParam,
            longParam,
        ) != 0 {
            return 1;
        }

        CallWindowProcA(
            ORIGINAL_WINDOW_PROCESS,
            WINDOW,
            message,
            wideParam,
            longParam,
        )
    }
}
