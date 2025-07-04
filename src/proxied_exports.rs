// Just proxied functions in this file
use proxygen_macros::forward;

#[forward]
#[export_name="GetFileVersionInfoA"]
pub extern "C" fn GetFileVersionInfoA() {}

#[forward]
#[export_name="GetFileVersionInfoByHandle"]
pub extern "C" fn GetFileVersionInfoByHandle() {}

#[forward]
#[export_name="GetFileVersionInfoExA"]
pub extern "C" fn GetFileVersionInfoExA() {}

#[forward]
#[export_name="GetFileVersionInfoExW"]
pub extern "C" fn GetFileVersionInfoExW() {}

#[forward]
#[export_name="GetFileVersionInfoSizeA"]
pub extern "C" fn GetFileVersionInfoSizeA() {}

#[forward]
#[export_name="GetFileVersionInfoSizeExA"]
pub extern "C" fn GetFileVersionInfoSizeExA() {}

#[forward]
#[export_name="GetFileVersionInfoSizeExW"]
pub extern "C" fn GetFileVersionInfoSizeExW() {}

#[forward]
#[export_name="GetFileVersionInfoSizeW"]
pub extern "C" fn GetFileVersionInfoSizeW() {}

#[forward]
#[export_name="GetFileVersionInfoW"]
pub extern "C" fn GetFileVersionInfoW() {}

#[forward]
#[export_name="VerFindFileA"]
pub extern "C" fn VerFindFileA() {}

#[forward]
#[export_name="VerFindFileW"]
pub extern "C" fn VerFindFileW() {}

#[forward]
#[export_name="VerInstallFileA"]
pub extern "C" fn VerInstallFileA() {}

#[forward]
#[export_name="VerInstallFileW"]
pub extern "C" fn VerInstallFileW() {}

#[forward]
#[export_name="VerLanguageNameA"]
pub extern "C" fn VerLanguageNameA() {}

#[forward]
#[export_name="VerLanguageNameW"]
pub extern "C" fn VerLanguageNameW() {}

#[forward]
#[export_name="VerQueryValueA"]
pub extern "C" fn VerQueryValueA() {}

#[forward]
#[export_name="VerQueryValueW"]
pub extern "C" fn VerQueryValueW() {}

