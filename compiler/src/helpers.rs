use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use file_system::errors::FileSystemError;

use crate::builder::get_zig_path;

pub fn bin_create_dir() -> Result<PathBuf, FileSystemError> {
    let global;
    if let Some(path) = env::current_dir()?.to_str() {
        global = format!("{path}/bin");
    } else {
        global = String::from("/bin");
    }
    let bin_path = Path::new(&global);
    if !bin_path.exists() {
        fs::create_dir(bin_path)?; //TODO: Burada Create Olunmama ehtimalı var onuda Error handling
        //et.
    }
    let zig_path = get_zig_path();
    Command::new(zig_path)
        .arg("inits")
        .current_dir(bin_path)
        .output()?;
    //TODO: Burada Status Code görə Error Handling Et.
    let deps_src = Path::new("./dependencies");
    let deps_dest = bin_path.join("dependencies");
    if deps_src.exists() {
        copy_dir_all(deps_src, &deps_dest)?;
    }

    Ok(bin_path.to_path_buf())
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), FileSystemError> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
