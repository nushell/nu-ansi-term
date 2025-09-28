#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("kernel32.dll" "system" fn CreateFileW(lpfilename : PCWSTR, dwdesiredaccess : u32, dwsharemode : FILE_SHARE_MODE, lpsecurityattributes : *const SECURITY_ATTRIBUTES, dwcreationdisposition : FILE_CREATION_DISPOSITION, dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES, htemplatefile : HANDLE) -> HANDLE);
windows_link::link!("kernel32.dll" "system" fn GetConsoleMode(hconsolehandle : HANDLE, lpmode : *mut CONSOLE_MODE) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn GetLastError() -> WIN32_ERROR);
windows_link::link!("kernel32.dll" "system" fn SetConsoleMode(hconsolehandle : HANDLE, dwmode : CONSOLE_MODE) -> BOOL);
pub type BOOL = i32;
pub type CONSOLE_MODE = u32;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: CONSOLE_MODE = 4u32;
pub type FILE_ACCESS_RIGHTS = u32;
pub type FILE_CREATION_DISPOSITION = u32;
pub type FILE_FLAGS_AND_ATTRIBUTES = u32;
pub const FILE_GENERIC_READ: FILE_ACCESS_RIGHTS = 1179785u32;
pub const FILE_GENERIC_WRITE: FILE_ACCESS_RIGHTS = 1179926u32;
pub type FILE_SHARE_MODE = u32;
pub const FILE_SHARE_WRITE: FILE_SHARE_MODE = 2u32;
pub type HANDLE = *mut core::ffi::c_void;
pub const INVALID_HANDLE_VALUE: HANDLE = -1i32 as _;
pub const OPEN_EXISTING: FILE_CREATION_DISPOSITION = 3u32;
pub type PCWSTR = *const u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: BOOL,
}
impl Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WIN32_ERROR = u32;
