use std::fs;

#[cfg(unix)]
use std::os::unix::fs::symlink as unix_symlink;
#[cfg(windows)]
use std::os::windows::fs::{symlink_dir, symlink_file};

fn deref(to_be_dereferenced: String) -> String {
    let mut metadata = fs::metadata(&to_be_dereferenced).unwrap();
    let file = &to_be_dereferenced;
    let mut new_file = file.clone();
    while metadata.is_symlink() {
        new_file = String::from(fs::read_link(new_file.to_string()).unwrap().into_os_string().into_string().unwrap());
        metadata = fs::metadata(&new_file).unwrap()
    }

    new_file
}

#[cfg(windows)]
pub fn symlink(to_be_linked: String, destination: String, dereference: bool) -> std::io::Result<()> {
    if !dereference {
        let target_metadata = fs::metadata(&to_be_linked)?;
        if target_metadata.is_dir() {
            symlink_dir(&to_be_linked, &destination)
        } else {
            symlink_file(&to_be_linked, &destination)
        }?;
    } else {
        let dereferenced = deref(to_be_linked.clone());
        let deref_metadata = fs::metadata(&dereferenced)?;
        if deref_metadata.is_dir() {
            symlink_dir(&dereferenced, &destination)
        } else {
            symlink_file(&dereferenced, &destination)
        }?;
    }
    Ok(())
}

#[cfg(unix)]
pub fn symlink(to_be_linked: String, destination: String, dereference: bool) -> std::io::Result<()> {
    if !dereference {
        unix_symlink(to_be_linked, destination)?;
    } else {
        let dereferenced = deref(to_be_linked);
        unix_symlink(dereferenced, destination)?;
    }
    Ok(())
}

pub fn hard_link(to_be_linked: String, destination: String, dereference: bool) -> std::io::Result<()> {
    if !dereference {
        fs::hard_link(to_be_linked, destination)?;
    } else {
        let dereferenced = deref(to_be_linked);
        fs::hard_link(dereferenced, destination)?;
    }
    Ok(())
}
