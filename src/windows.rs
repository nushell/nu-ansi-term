// ref: https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#EXAMPLE_OF_ENABLING_VIRTUAL_TERMINAL_PROCESSING @@ https://archive.is/L7wRJ#76%
use crate::win_bindings::{
    CreateFileW, GetConsoleMode, GetLastError, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
    FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_WRITE, INVALID_HANDLE_VALUE, OPEN_EXISTING,
};

/// Enables ANSI code support on Windows 10.
///
/// This uses Windows API calls to alter the properties of the console that
/// the program is running in.
///
/// https://msdn.microsoft.com/en-us/library/windows/desktop/mt638032(v=vs.85).aspx
///
/// Returns a `Result` with the Windows error code if unsuccessful.
pub fn enable_ansi_support() -> Result<(), u32> {
    unsafe {
        // ref: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew
        // Using `CreateFileW("CONOUT$", ...)` to retrieve the console handle works correctly even if STDOUT and/or STDERR are redirected
        let console_handle = CreateFileW(
            w!("CONOUT$"),
            FILE_GENERIC_READ | FILE_GENERIC_WRITE,
            FILE_SHARE_WRITE,
            std::ptr::null_mut(), // SECURITY_ATTRIBUTES
            OPEN_EXISTING,
            0,                    // FILE_FLAGS_AND_ATTRIBUTES
            std::ptr::null_mut(), // hTemplateFile: HANDLE
        );
        if console_handle == INVALID_HANDLE_VALUE {
            return Err(GetLastError());
        }

        // ref: https://docs.microsoft.com/en-us/windows/console/getconsolemode
        let mut console_mode = 0;
        if 0 == GetConsoleMode(console_handle, &mut console_mode) {
            return Err(GetLastError());
        }

        // VT processing not already enabled?
        if console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0 {
            // https://docs.microsoft.com/en-us/windows/console/setconsolemode
            if 0 == SetConsoleMode(
                console_handle,
                console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING,
            ) {
                return Err(GetLastError());
            }
        }

        Ok(())
    }
}

const CONOUT_WSTR: &[u16] = &[
    36,
    b'C' as u16,
    b'O' as u16,
    b'N' as u16,
    b'O' as u16,
    b'U' as u16,
    b'T' as u16,
    b'$' as u16,
    0, // Null terminator
];
