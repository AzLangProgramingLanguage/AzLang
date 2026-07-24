use std::path::PathBuf;
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
pub fn get_zig_path() -> &'static str {
    "zig"
}
const BLUE: &str = "\x1b[94m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const MAGENTA: &str = "\x1b[95m";
const WHITE: &str = "\x1b[97m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

pub fn build(source: PathBuf) -> Result<(), CompilerError> {
    let output_path = source.join(format!(
        "zig-out/bin/bin{}",
        if cfg!(target_os = "windows") {
            ".exe"
        } else {
            ""
        }
    ));

    let zig_path = get_zig_path();
    let compile_status = Command::new(zig_path)
        .arg("build")
        .arg("run")
        .current_dir(source)
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
        Err(e) => {
            println!("{e}");
            Err(CompilerError::BuildError)
        }
    }
}
