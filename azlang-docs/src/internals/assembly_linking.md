# Direct Assembly & Linking

The final stage of the AzLang compiler pipeline turns intermediate code into a finished, standalone bare-metal executable.

---

## 1. Pipeline Execution Flow

AzLang produces machine code through a deterministic three-step lowering pipeline:

1. **Intermediate Lowering**: Source files compile directly to QBE Intermediate Representation (`.il`).
2. **Assembly Emission**: The QBE backend emits GNU assembly (`.s`), which is assembled into an object file (`.o`).
3. **Static Linking**: The object file is linked together with any `@link` assembly or C objects via `ld.lld` into the final standalone executable (`app`).

---

## 2. Linker Invocation & Target Binary

Because AzLang produces code directly targeting the system entry point `_start` and links custom assembly stubs (`write.o`, `exit.o`), no C runtime or dynamic linker flags are required:

```bash
ld.lld ./sdk/build/write.o ./sdk/build/exit.o main.o -o app
```

The resulting `app` is a self-contained ELF executable with:
- **No dynamic interpreter** (e.g. `/lib64/ld-linux-x86-64.so.2` is not required)
- **Sub-3KB binary size**
- **Instantaneous microsecond startup**
