# Hello World Walkthrough

Here is a step-by-step walkthrough of writing and compiling your first native AzLang program using the real SDK syntax.

---

## 1. Writing `main.az`

Create a file named `main.az`:

```azlang
enum FD
    Stdin
    Stdout
    Stderr

@link("./sdk/src/write.s")
op write(FD fd, const str val, const int size): void

@link("./sdk/src/exit.s")
op exit(const int val): void

write(Stdout, "Hello World from AzLang!\n", 25)
exit(0)
```

### Deconstructing the Code:
1. `enum FD`: Defines standard file descriptors (`Stdin=0`, `Stdout=1`, `Stderr=2`). Enum variants are inlined as immediates — no lookup table exists at runtime.
2. `@link(...)`: Points at a fasm assembly source containing a syscall stub that performs a direct Linux `syscall` instruction. The source is included straight into the generated assembly — there is nothing to pre-compile.
3. `op write(...)`: Declares a callable operation function signature and parameter types.
4. `write(...)` & `exit(...)`: Calls the operations natively without libc overhead. The string length `25` accounts for the `\n` escape, which is resolved to a single newline byte at emission time.

---

## 2. Compiling with AzCLI

Compile `main.az` using `azcli`:

```bash
azcli build main.az
```

The AzLang pipeline will:
1. Parse `main.az` into an AST.
2. Validate symbols and types.
3. Emit a single assembly file (`main.asm`) containing the program code, its string data, and `include` directives for `write.s` and `exit.s`.
4. Invoke `fasm`, which assembles and links that file directly into the standalone binary `main` in one pass.

> The only external tool required is **[fasm](https://flatassembler.net/)**. There is no `qbe`, no `as`, no `ld` and no object files.

Run the resulting executable:

```bash
./main
# Output:
# Hello World from AzLang!
```

Check the binary size:
```bash
ls -l main
# 228 bytes
```
A complete, functional ELF executable in 228 bytes!
