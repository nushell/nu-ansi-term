use std::fs;

use windows_bindgen::bindgen;

#[test]
fn gen_bindings() {
    let existing = fs::read_to_string(BINDINGS).unwrap();

    bindgen(INPUT).unwrap();

    // Check the output is the same as before.
    // Depending on the git configuration the file may have been checked out with `\r\n` newlines or
    // with `\n`. Compare line-by-line to ignore this difference.
    let mut new = fs::read_to_string(BINDINGS).unwrap();
    if existing.contains("\r\n") && !new.contains("\r\n") {
        new = new.replace("\n", "\r\n");
    } else if !existing.contains("\r\n") && new.contains("\r\n") {
        new = new.replace("\r\n", "\n");
    }

    assert_eq!(existing, new);
    if !new.lines().eq(existing.lines()) {
        panic!("generated file `{BINDINGS}` has been updated");
    }
}

const INPUT: &[&str] = &[
    "--no-deps",
    "--etc",
    "--out",
    BINDINGS,
    "--flat",
    "--sys",
    "--no-comment",
    "--filter",
    "GetLastError",
    "INVALID_HANDLE_VALUE",
    "CreateFileW",
    "FILE_GENERIC_READ",
    "FILE_GENERIC_WRITE",
    "FILE_SHARE_WRITE",
    "OPEN_EXISTING",
    "GetConsoleMode",
    "SetConsoleMode",
    "ENABLE_VIRTUAL_TERMINAL_PROCESSING",
];

const BINDINGS: &str = "src/win_bindings.rs";
