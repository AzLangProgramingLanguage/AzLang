use std::io;
use std::path::Path;
use std::process::Command;

use crate::errors::CompilerError;
/* TODO: PathBuf a gələcəkdə baxarsan */
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
const BLUE: &str = "\x1b[94m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const MAGENTA: &str = "\x1b[95m";
const WHITE: &str = "\x1b[97m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

pub fn build(zig_file: &str, output_file: &str) -> Result<(), CompilerError> {
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

    let zig_path = get_zig_path();
    let compile_status = Command::new(zig_path)
        .arg("build-exe")
        .arg(zig_file)
        .arg(format!("-femit-bin={}", output_path.to_str().unwrap()))
        .status();
    match compile_status {
        Ok(status) => {
            if status.success() {
                println!("🚀 Yığım tamamlandı. Proqram istifadə üçün hazırdır:\n");
                println!("\n{BLUE}=============================== {RESET}");
                println!("{GREEN}{BOLD}🚀 Azlang Build Complete! ✔{RESET}");
                println!("{BLUE}=============================== {RESET}");
                println!("{WHITE}Program uğurla yığıldı və hazırdır:{RESET}");

                match output_path.canonicalize() {
                    Ok(abs) => println!(
                        "{BLUE}📁 Output File:{RESET} {YELLOW}{BOLD}{}{RESET}",
                        abs.display()
                    ),
                    Err(_) => println!(
                        "{BLUE}📁 Output File:{RESET} {YELLOW}{BOLD}{}{RESET}",
                        output_path.display()
                    ),
                }

                println!("{BLUE}=============================== {RESET}");
                println!("{MAGENTA}{BOLD}🔥 Run it with:{RESET}");
                println!(
                    "{MAGENTA}   →{RESET} {YELLOW}{BOLD}{}{RESET}",
                    output_path.display()
                );
                println!("{BLUE}===============================\n{RESET}");
                Ok(())
            } else {
                Err(CompilerError::BuildError)
            }
        }
        Err(_) => Err(CompilerError::BuildError),
    }
}
