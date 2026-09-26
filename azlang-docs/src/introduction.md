# Introduction to AzLang

> **AzLang** is a minimal, blazingly fast, and readable systems programming language engineered for direct bare-metal execution, sub-3KB binary output, and zero-runtime overhead.

AzLang blends **Python-like visual clarity**, **Rust-like memory control**, and **TypeScript-like type safety**, stripping away heavy runtime abstractions and external runtime daemons.

---

## Key Characteristics

- **Zero-VM Execution**: No virtual machines, dynamic garbage collectors, or heavy runtime initialization routines (`crt1.o`). Execution lowers directly to native machine code.
- **Microsecond Compilations**: A lean frontend passes verified AST directly to intermediate representation and native machine instructions.
- **Lean Binary Footprint**: Produces standalone, self-contained ELF executables often measuring between **2KB and 3KB**.
- **Absence of Moving Parts**: Eliminates runtime state machines, dynamic heap leaks, and unwinding tables by design.
- **Direct ABI & Syscall Interop**: High-performance systems interfaces via native `@link` decorators and direct kernel syscall invocation (`x86_64` register calling conventions).

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
│  [Direct AST Lowering / QBE IR (.il)]                                             │
│          │                                                                        │
│          ▼                                                                        │
│  [Native Assembly Generation (.s)] ──► [Assembler: as] ──► Object (.o)            │
│                                                                  │                │
│                                                          [Direct Syscalls]        │
│                                                                  │                │
│                                                                  ▼                │
│                                                     [Linker: ld.lld]              │
│                                                                  │                │
│                                                                  ▼                │
│                                                      [~2.6 KB Bare-Metal Binary]  │
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

@link("./sdk/build/write.o")
op write(FD fd, const str val, const int size): void

@link("./sdk/build/exit.o")
op exit(const int val): void

write(Stdout, "Hello, Bare Metal AzLang!\n", 26)
exit(0)
```
