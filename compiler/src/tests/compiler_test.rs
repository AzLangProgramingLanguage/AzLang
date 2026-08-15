use std::{path::PathBuf, process::Command};

use which::which;

use crate::{
    errors::{BackendError, CompilerError},
    libc_checker, parser,
};
/*
*
*  I change my mind,  github actions will not test this code.
*
* */
#[test]
fn dependencies_test() -> Result<(), CompilerError> {
    which("qbe").map_err(|_| CompilerError::Backend(BackendError::Qbe))?;
    which("as").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    which("ld").map_err(|_| CompilerError::Backend(BackendError::BinUtils))?;
    Ok(())
}
#[test]
fn compiler_output_file() -> Result<(), CompilerError> {
    let linker = libc_checker::libc_link_checker().expect("Error");

    let main_file: PathBuf = PathBuf::from("main.ssa");
    file_system::write_file(
        &main_file,
        "
function w $add(w %a, w %b) {              # Define a function add
@start
	%c =w add %a, %b                   # Adds the 2 arguments
	ret %c                             # Return the result
}

            export function w $main() {                # Main function
    @start
     	  %r =w call $add(w 1, w 1)          # Call add(1, 1)
     	  call $printf(l $fmt, ..., w %r)    # Show the result
     	  ret 0
    }
data $fmt = { b \"One and one make %d!\n\", b 0 }
     "
        .to_string(),
    )?;
    Command::new("qbe")
        .args(["-o", "main.s", "main.ssa"])
        .status()
        .expect("Error");
    Command::new("as")
        .args(["main.s", "-o", "main.o"])
        .status()
        .expect("Assembler  can't compile to object ");
    Command::new(linker)
        .args([
            "-dynamic-linker",
            "/lib64/ld-linux-x86-64.so.2",
            "/usr/lib/Scrt1.o",
            "main.o",
            "-L/usr/lib",
            "-lc",
            "/usr/lib/crtn.o",
            "-o",
            "app",
        ])
        .status()
        .expect("Error");
    Ok(())
}
