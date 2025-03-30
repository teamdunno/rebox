#![cfg(unix)]

use std::fs;
use std::process::Command;

/// A safe wrapper for checking if the user is root.
struct UserPrivileges;

impl UserPrivileges {
    fn is_root() -> bool {
        if let Ok(status) = fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("Uid:") {
                    let uid: u32 = line
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or("1")
                        .parse()
                        .unwrap_or(1);
                    return uid == 0;
                }
            }
        }

        // Fallback: Run `id -u`
        Command::new("id")
            .arg("-u")
            .output()
            .map(|o| o.stdout == b"0\n")
            .unwrap_or(false)
    }
}

/// Safe function to check if the user has root privileges.
pub fn is_admin() -> bool {
    UserPrivileges::is_root()
}

pub fn get_username() -> Option<String> {
    std::env::var("USER").ok()
}
