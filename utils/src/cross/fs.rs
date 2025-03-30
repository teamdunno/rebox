use std::fs;

#[cfg(unix)]
use std::os::unix::fs::symlink as unix_symlink;
#[cfg(windows)]
use std::os::windows::fs::{symlink_dir, symlink_file};

#[cfg(windows)]
pub fn symlink(to_be_linked: String, destination: String) -> std::io::Result<()> {
    let target_metadata = fs::metadata(&to_be_linked)?;
    if target_metadata.is_dir() {
        symlink_dir(&to_be_linked, &destination)
    } else {
        symlink_file(&to_be_linked, &destination)
    }?;
    Ok(())
}

#[cfg(unix)]
pub fn symlink(to_be_linked: String, destination: String) -> std::io::Result<()> {
    unix_symlink(to_be_linked, destination)?;
    Ok(())
}

pub fn hard_link(to_be_linked: String, destination: String) -> std::io::Result<()> {
    fs::hard_link(to_be_linked, destination)?;
    Ok(())
}
