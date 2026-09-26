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

@link("./sdk/build/write.o")
op write(FD fd, const str val, const int size): void

@link("./sdk/build/exit.o")
op exit(const int val): void

write(Stdout, "Hello World from AzLang!\n", 25)
exit(0)
```

### Deconstructing the Code:
1. `enum FD`: Defines standard file descriptors (`Stdin=0`, `Stdout=1`, `Stderr=2`).
2. `@link(...)`: Links directly to an object file (`write.o` / `exit.o`) compiled from assembly stubs that perform direct Linux syscalls (`syscall` instruction).
3. `op write(...)`: Declares a callable operation function signature and parameter types.
4. `write(...)` & `exit(...)`: Calls the operations natively without libc overhead.

---

## 2. Compiling the SDK Assembly Stubs

If you are working inside the repository or using the SDK:

```bash
# Build the assembly syscall objects
as sdk/src/write.s -o sdk/build/write.o
as sdk/src/exit.s -o sdk/build/exit.o
```

---

## 3. Compiling and Running with AzCLI

Compile `main.az` using `azcli`:

```bash
azcli build main.az
```

The AzLang pipeline will:
1. Parse `main.az` into an AST.
2. Validate symbols and types.
3. Emit QBE Intermediate Language (`main.il`).
4. Invoke `qbe` to produce `main.s`.
5. Invoke `as` to produce `main.o`.
6. Invoke `ld.lld` to link `main.o` together with `write.o` and `exit.o` into the standalone binary `app`.

> The backend tools (`qbe`, `as`, `ld.lld`) are pulled in automatically by the `azlang` package. You never need to install or call them yourself.

Run the resulting executable:

```bash
./app
# Output:
# Hello World from AzLang!
```

Check the binary size:
```bash
ls -lh app
# Output: ~2.6K
```
A complete, functional ELF binary in less than 3 kilobytes!
