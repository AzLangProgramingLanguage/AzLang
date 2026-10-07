# Introduction to AzLang

> **AzLang** is a minimal, blazingly fast, and readable systems programming language engineered for direct bare-metal execution, sub-kilobyte binary output, and zero-runtime overhead.

AzLang blends **Python-like visual clarity**, **Rust-like memory control**, and **TypeScript-like type safety**, stripping away heavy runtime abstractions and external runtime daemons.

---

## Key Characteristics

- **Zero-VM Execution**: No virtual machines, dynamic garbage collectors, or heavy runtime initialization routines (`crt1.o`). Execution lowers directly to native machine code.
- **Microsecond Compilations**: A lean frontend passes verified AST directly to fasm assembly and native machine instructions in a single pass.
- **Lean Binary Footprint**: Produces standalone, self-contained ELF executables — a hello world program measures **228 bytes**, and a comprehensive multi-module program stays around **2 KB**.
- **Absence of Moving Parts**: Eliminates runtime state machines, dynamic heap leaks, and unwinding tables by design.
- **Direct ABI & Syscall Interop**: High-performance systems interfaces via native `@link` directives and direct kernel syscall invocation (`x86_64` register calling conventions).

---

## Architectural Snapshot

```text
┌───────────────────────────────────────────────────────────────────────────────────┐
│ AZLANG ARCHITECTURE                                                               │
│                                                                                   │
│  [Source Code (.az)]                                                              │
│          │                                                                        │
│          ▼                                                                        │
│  [Tokenizer & Indent Engine]                                                      │
│          │                                                                        │
│          ▼                                                                        │
│  [AST Construction & Validation]                                                  │
│          │                                                                        │
│          ▼                                                                        │
│  [Transpiler → fasm Assembly (.asm)] ──► @link syscall stubs included inline      │
│          │                                                                        │
│          ▼                                                                        │
│  [fasm: single-pass assembler & linker]                                           │
│          │                                                                        │
│          ▼                                                                        │
│                                                  [~228 Byte Bare-Metal ELF]       │
└───────────────────────────────────────────────────────────────────────────────────┘
```

---

## Real Syntax At a Glance

Here is an authentic snippet from the AzLang SDK illustrating low-level declarations, enumerations, ABI linking, and entry invocations:

```azlang
enum FD
    Stdin
    Stdout
    Stderr

@link("./sdk/src/write.s")
op write(FD fd, const str val, const int size): void

@link("./sdk/src/exit.s")
op exit(const int val): void

write(Stdout, "Hello, Bare Metal AzLang!\n", 26)
exit(0)
```
