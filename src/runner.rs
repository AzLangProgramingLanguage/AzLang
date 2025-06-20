use std::io;
use std::process::Command;

pub fn compile_and_run(rust_file: &str) -> Result<(), io::Error> {
    let compile_status = Command::new("zig").arg("run").arg(rust_file).status()?;

    if compile_status.success() {
        println!("🚀 Kompilyasiya uğurla tamamlandı. Proqram başladı:\n");
        /*
        let run_status = Command::new(output_exe).status()?;

        if !run_status.success() {
            eprintln!("⚠️ Proqram işləyəndə səhv çıxdı.");
        } */
        Ok(())
    } else {
        eprintln!("❌ Kompilyasiya xətası!");
        Err(io::Error::new(io::ErrorKind::Other, "Kompilyasiya xətası"))
    }
}
