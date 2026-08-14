use std::path::Path;

pub fn libc_link_checker() -> Result<&'static str, &'static str> {
    if Path::new("/usr/lib/Scrt1.o").exists()
        && Path::new("/usr/lib/crtn.o").exists()
        && Path::new("/lib64/ld-linux-x86-64.so.2").exists()
    {
        return Ok("/usr/bin/ld");
        // return Ok(format!(
        //             "
        //    ld \
        //                                 -dynamic-linker /lib64/ld-linux-x86-64.so.2 \
        //                                 /usr/lib/Scrt1.o \
        //                                 file.o \
        //                                 -L/usr/lib \
        //                                 -lc \
        //                                 azrt.o \
        //                                 printf.o \
        //                                 /usr/lib/crtn.o \
        //                                 -o app
        //
        // "
        // ));
    }
    Err("In your system has not libc ")
}
