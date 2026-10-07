# Kernel Syscalls & Bare-Metal Stubs

In the AzLang architecture, standard I/O does not rely on the heavy GNU C Library (`glibc` or `musl`). Instead, core operations are mapped directly to Linux kernel system calls using lean assembly stubs.

---

## 1. System Call Assembly Implementations

The AzLang SDK provides plain fasm assembly sources for Linux `x86_64` (fasm defaults to Intel syntax). These files are pulled in by `@link` via `include` — they are never assembled on their own.

### `write.s` — Direct `sys_write` (Syscall 1)
```asm
write:
    mov rax, 1      ; sys_write
    syscall
    ret
```

On `x86_64` Linux ABI:
- `rdi`: File descriptor (`FD fd`)
- `rsi`: Buffer pointer (`const str val`)
- `rdx`: Buffer byte count (`const int size`)
- `rax`: Syscall number (`1` for `sys_write`)

### `exit.s` — Direct `sys_exit` (Syscall 60)
```asm
exit:
    mov rax, 60     ; sys_exit
    syscall
```

### `fopen.s` — Direct `sys_open` (Syscall 2)
```asm
fopen:
    mov rax, 2      ; sys_open
    syscall
    ret
```

---

## 2. Advantages of Direct Syscalls

- **Zero Startup Penalty**: No dynamic symbol relocations or runtime linker (`ld-linux.so`) overhead.
- **Microscopic Footprint**: Omits megabytes of standard library code that standard C programs pull in.
- **Security & Attack Surface Reduction**: Eliminates vulnerabilities inside complex libc string/format-string/locale handlers.
