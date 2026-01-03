#![feature(naked_functions)]
#![allow(named_asm_labels)]
#![allow(non_snake_case)]
// #![feature(asm_const)]

mod export_indices;
mod intercepted_exports;
mod orig_exports;
mod proxied_exports;

#[allow(unused_imports)]
pub use intercepted_exports::*;
pub use proxied_exports::*;

use export_indices::TOTAL_EXPORTS;
use orig_exports::load_dll_funcs;
#[cfg(target_arch="x86_64")]
use std::arch::x86_64::_mm_pause;
#[cfg(target_arch="x86")]
use std::arch::x86::_mm_pause;
use std::ffi::OsString;
use std::os::windows::prelude::{AsRawHandle, OsStringExt};
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{FARPROC, HINSTANCE, HMODULE, MAX_PATH};
use winapi::shared::ntdef::LPCSTR;
// use winapi::um::consoleapi::AllocConsole;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::{
    DisableThreadLibraryCalls, FreeLibrary, GetModuleFileNameW, LoadLibraryA,
};
use winapi::um::processenv::SetStdHandle;
use winapi::um::processthreadsapi::{
    CreateThread, GetCurrentProcess, TerminateProcess,
};
use winapi::um::winbase::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use winapi::um::winuser::{MessageBoxA, MB_OK};

// Static handles
static mut THIS_HANDLE: Option<HMODULE> = None;
static mut ORIG_DLL_HANDLE: Option<HMODULE> = None;

// Original funcs
#[no_mangle]
static mut ORIGINAL_FUNCS: [FARPROC; TOTAL_EXPORTS] = [std::ptr::null_mut(); TOTAL_EXPORTS];
#[no_mangle]
static mut ORIG_FUNCS_PTR: *const FARPROC = std::ptr::null_mut();

/// Indicates once we are ready to accept incoming calls to proxied functions
static mut PROXYGEN_READY: bool = false;

// Max path length is 32767
//https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation?tabs=registry
const STRING_BUFF_SIZE: usize = 32767;

fn get_current_process_name() -> Option<String> {
    // バッファを確保（MAX_PATH = 260）
    let mut buffer = [0u16; MAX_PATH];

    // NULL にすると現在のプロセスのモジュール
    let length = unsafe {
        GetModuleFileNameW(std::ptr::null_mut() as HINSTANCE,  &mut buffer[0], MAX_PATH.try_into().unwrap()) as usize
    };

    if length == 0 || length > buffer.len() {
        return None;
    }

    let os_string = OsString::from_wide(&buffer[..length]);
    os_string.into_string().ok()
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(module: HMODULE, reason: u32, _res: *const c_void) -> i32 {
    DisableThreadLibraryCalls(module);
    THIS_HANDLE = Some(module);

    match reason {
        DLL_PROCESS_ATTACH => {
                CreateThread(std::ptr::null_mut(), 0, Some(init), std::ptr::null_mut(), 0, std::ptr::null_mut());
        },
        DLL_PROCESS_DETACH  => {
            if let Some(orig_dll_handle) = ORIG_DLL_HANDLE {
                println!("Freeing original DLL");
                FreeLibrary(orig_dll_handle);
            }
        },
        _ => ()
    }

    1
}

/// Get the current DLLs path
unsafe fn get_dll_path() -> Option<String> {
    let mut buffer: Vec<u16> = vec![0; STRING_BUFF_SIZE];
    if THIS_HANDLE.is_none() {
        return None;
    }
    let size = GetModuleFileNameW(
        THIS_HANDLE.unwrap(),
        buffer.as_mut_ptr(),
        buffer.len() as u32,
    );

    if size == 0 {
        return None;
    }

    buffer.truncate(size as usize);
    let os_string = OsString::from_wide(&buffer);
    Some(os_string.to_string_lossy().into_owned())
}

unsafe fn die() {
    show_message("ARAD_PLUGIN_LOADER", "About to exit...");
    println!("Exiting...");
    TerminateProcess(GetCurrentProcess(), 0);
}

unsafe fn show_message(title: &str, message: &str) {
    let title = format!("{}\0", title);
    let message = format!("{}\0", message);
    MessageBoxA(
        std::ptr::null_mut(),
        message.as_bytes().as_ptr() as LPCSTR,
        title.as_bytes().as_ptr() as LPCSTR,
        MB_OK,
    );
}

/// Called when the thread is spawned
unsafe extern "system" fn init(_: *mut c_void) -> u32 {
    ORIG_FUNCS_PTR = ORIGINAL_FUNCS.as_ptr();
    // AllocConsole();
    let stdout = std::io::stdout();
    let out_handle = stdout.as_raw_handle();
    let out_handle = out_handle as *mut c_void;
    SetStdHandle(STD_OUTPUT_HANDLE, out_handle);
    let stderr = std::io::stderr();
    let err_handle = stderr.as_raw_handle();
    let err_handle = err_handle as *mut c_void;
    SetStdHandle(STD_ERROR_HANDLE, err_handle);
    if let Some(dll_path) = get_dll_path() {
        println!("This DLL path: {}", &dll_path);
        let orig_dll_name = format!("{}_\0", &dll_path);
        ORIG_DLL_HANDLE = Some(LoadLibraryA(orig_dll_name.as_ptr() as *const i8));
    } else {
        show_message("ARAD_PLUGIN_LOADER", "Failed to get DLL path");
        eprint!("Failed to get DLL path");
        return 1;
    }
    if let Some(orig_dll_handle) = ORIG_DLL_HANDLE {
        if orig_dll_handle.is_null() {
            let err = GetLastError();
            eprintln!("Failed to load original DLL");
            show_message(
                "ARAD_PLUGIN_LOADER",
                &format!("Failed to load original DLL. Error: {}", err),
            );
            die();
        }
        println!("Original DLL handle: {:?}", orig_dll_handle);
    } else {
        let err = GetLastError();
        eprintln!("Failed to load original DLL");
        show_message(
            "ARAD_PLUGIN_LOADER",
            &format!("Failed to load original DLL. Error: {}", err),
        );
        die();
    }
    load_dll_funcs();
    PROXYGEN_READY = true;

    let isAradGameProcess = get_current_process_name().unwrap().ends_with("ARAD.exe");
    if isAradGameProcess {
        load_plugin()
    }

    0
}

fn load_plugin() {
    let mut current_dir = std::env::current_dir().expect("現在のディレクトリを取得できませんでした");
    current_dir.push("plugins");

    for entry in std::fs::read_dir(current_dir).expect("ディレクトリの読み込みに失敗") {
        let entry = entry.expect("エントリ取得失敗");
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "dll" {
                    unsafe {
                        LoadLibraryA(path.to_str().unwrap().as_ptr() as *const i8);
                    }
                }
            }
        }
    }
}

/// Call this before attempting to call a function in the proxied DLL
/// 
/// This will wait for proxygen to fully load up all the proxied function addresses before returning
#[no_mangle]
pub extern "C" fn wait_dll_proxy_init() {
    // NOTE TO SELF: DO NO PRINT STUFF IN HERE
    
    // Safety: `PROXYGEN_READY` will only get flipped to true once, and never back again.
    // We also check if sse2 is supported before using _mm_pause
    if is_x86_feature_detected!("sse2") {
        unsafe {
            while !PROXYGEN_READY {
                _mm_pause();
            }
        }
    } else {
        while !unsafe { PROXYGEN_READY } {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
