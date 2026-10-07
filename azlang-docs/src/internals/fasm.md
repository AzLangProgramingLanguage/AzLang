# fasm Assembly Emission

AzLang compiles code by emitting a single, human-readable **fasm** (flat assembler) assembly file, which `fasm` turns directly into a native ELF executable in one pass.

---

## 1. Why fasm?

fasm was chosen because it aligns with AzLang's minimalist engineering doctrine:
- **One Pass, One Binary**: fasm assembles *and* links in a single step — there is no separate `as`, no `ld`, no object files.
- **Small Footprint**: the entire fasm distribution is a single binary of well under 1 MB.
- **Readable Output**: the emitted `.asm` is clean, commented-friendly x86_64 assembly you can inspect directly.

---

## 2. Sample Assembly Emission

Here is the exact assembly generated for the [Hello World](../getting-started/hello_world.md) program (paths abbreviated; the compiler canonicalizes them to absolute paths):

```asm
format ELF64 executable
entry _start

segment readable executable

include "./sdk/src/write.s"
include "./sdk/src/exit.s"

_start:
    push rbp
    mov rbp, rsp
    sub rsp, 8
    mov rax, 1
    push rax
    lea rax, [main_str0]
    push rax
    mov rax, 25
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    mov rax, 0
    push rax
    pop rdi
    call exit
    xor edi, edi
    mov eax, 60
    syscall

main_str0 db "Hello World from AzLang!",10,0
```

AzLang generates this directly from the validated AST:
- Function calls lower to ordinary `call <symbol>` instructions following the System V AMD64 ABI (register arguments, 16-byte stack alignment).
- String literals become `db` data directives (escape sequences such as `\n` are resolved to bytes at emission time).
- `@link(...)` sources that are actually invoked are pulled in with `include`, so unused SDK stubs never reach the binary.
- Programs end with a direct `sys_exit` (`rax = 60`) — no C runtime, no `ret` to nowhere.
