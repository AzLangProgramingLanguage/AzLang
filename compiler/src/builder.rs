use std::io;
use std::path::Path;
use std::process::Command;
/* fn get_zig_path() -> PathBuf {
    let exe_path = env::current_exe().unwrap(); // azcli.exe
    let bin_dir = exe_path.parent().unwrap(); // .azlang/bin
    bin_dir
        .join("dependencies")
        .join(if cfg!(windows) { "zig.exe" } else { "zig" })
} */
fn get_zig_path() -> &'static str {
    "zig"
}

pub fn build(rust_file: &str, output_file: &str) -> Result<(), io::Error> {
    let parent_dir = Path::new(output_file)
        .parent()
        .unwrap_or_else(|| Path::new("."));

    let output_path = parent_dir.join(format!(
        "program.{}",
        if cfg!(target_os = "windows") {
            "exe"
        } else {
            ""
        }
    ));

    println!("🚀 Yığım tamamlandı. Proqram istifadə üçün hazırdır:\n");
    let zig_path = get_zig_path();
    let compile_status = Command::new(zig_path)
        .arg("build-exe")
        .arg(rust_file)
        .arg(format!("-femit-bin={}", output_path.to_str().unwrap()))
        .status()?;

    if compile_status.success() {
        Ok(())
    } else {
        eprintln!("❌ Kompilyasiya xətası!");
        Err(io::Error::other("Kompilyasiya xətası"))
    }
}
