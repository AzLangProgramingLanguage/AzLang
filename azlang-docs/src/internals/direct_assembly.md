# Single-Pass Assembly & Binary Layout

The final stage of the AzLang compiler pipeline turns the emitted assembly into a finished, standalone bare-metal executable — in one invocation, with no linker.

---

## 1. Pipeline Execution Flow

AzLang produces machine code through a deterministic two-step lowering pipeline:

1. **Assembly Emission**: the validated AST is lowered into one `.asm` file — header, `include` directives for every invoked `@link` source, the program's code, and its string data.
2. **Single-Pass Assembly**: `fasm` assembles that file directly into the final ELF executable. Assembly and linking happen in the same pass, so no intermediate object files ever exist.

```bash
fasm main.asm main
```

Because `@link` targets are fasm assembly *sources* (not pre-built `.o` files), there is nothing to pre-compile: the symbols `write`, `exit`, etc. are defined in the same translation unit and resolved statically at assembly time.

---

## 2. Resulting Binary

AzLang emits `format ELF64 executable` with `entry _start` and a single `readable executable` segment (a 120-byte program header). Because the program targets the system entry point `_start` and inlines its syscall stubs, no C runtime or dynamic linker is involved:

- **No dynamic interpreter** (`/lib64/ld-linux-x86-64.so.2` is not required)
- **No section headers, no relocations** — a minimal static image
- **Tiny footprint**: the Hello World program measures **228 bytes**; a comprehensive 60-line multi-module program stays around **2 KB**
- **Instantaneous startup**: execution begins at `_start` with direct syscalls

```text
$ file main
main: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, no section header
$ ./main
Hello World from AzLang!
```
