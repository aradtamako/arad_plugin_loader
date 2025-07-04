use crate::export_indices::*;
use crate::{ORIGINAL_FUNCS, ORIG_DLL_HANDLE};
use std::ffi::CString;
use winapi::{
    shared::minwindef::{FARPROC, HMODULE},
    um::libloaderapi::GetProcAddress,
};

/// Loads up the address of the original function in the given module
unsafe fn load_dll_func(index: usize, h_module: HMODULE, func: &str) {
    let func_c_string = CString::new(func).unwrap();
    let proc_address: FARPROC = GetProcAddress(h_module, func_c_string.as_ptr());
    ORIGINAL_FUNCS[index] = proc_address;
    println!("[0x{:016x}] Loaded {}", proc_address as u64, func);
}

/// Loads the original DLL functions for later use
pub unsafe fn load_dll_funcs() {
    println!("Loading original DLL functions");
    if ORIG_DLL_HANDLE.is_none() {
        eprintln!("Original DLL handle is none. Cannot load original DLL funcs");
        return;
    }
    let dll_handle = ORIG_DLL_HANDLE.unwrap();
    load_dll_func(Index_GetFileVersionInfoA, dll_handle, "GetFileVersionInfoA");
    load_dll_func(Index_GetFileVersionInfoByHandle, dll_handle, "GetFileVersionInfoByHandle");
    load_dll_func(Index_GetFileVersionInfoExA, dll_handle, "GetFileVersionInfoExA");
    load_dll_func(Index_GetFileVersionInfoExW, dll_handle, "GetFileVersionInfoExW");
    load_dll_func(Index_GetFileVersionInfoSizeA, dll_handle, "GetFileVersionInfoSizeA");
    load_dll_func(Index_GetFileVersionInfoSizeExA, dll_handle, "GetFileVersionInfoSizeExA");
    load_dll_func(Index_GetFileVersionInfoSizeExW, dll_handle, "GetFileVersionInfoSizeExW");
    load_dll_func(Index_GetFileVersionInfoSizeW, dll_handle, "GetFileVersionInfoSizeW");
    load_dll_func(Index_GetFileVersionInfoW, dll_handle, "GetFileVersionInfoW");
    load_dll_func(Index_VerFindFileA, dll_handle, "VerFindFileA");
    load_dll_func(Index_VerFindFileW, dll_handle, "VerFindFileW");
    load_dll_func(Index_VerInstallFileA, dll_handle, "VerInstallFileA");
    load_dll_func(Index_VerInstallFileW, dll_handle, "VerInstallFileW");
    load_dll_func(Index_VerLanguageNameA, dll_handle, "VerLanguageNameA");
    load_dll_func(Index_VerLanguageNameW, dll_handle, "VerLanguageNameW");
    load_dll_func(Index_VerQueryValueA, dll_handle, "VerQueryValueA");
    load_dll_func(Index_VerQueryValueW, dll_handle, "VerQueryValueW");
}
