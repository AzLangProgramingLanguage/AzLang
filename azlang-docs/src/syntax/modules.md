# Modules & Imports

AzLang organizes codebases into self-contained modules imported using the `import` statement.

---

## 1. Importing Modules

To import definitions from another `.az` file:

```azlang
import lib

write(Stdout, "Hello from imported module\n", 27)
```

In the AzLang SDK, `sample.az` imports `lib.az`:

```azlang
// sample.az
import lib

write(Stdout, "Hello World", 11)
```

Where `lib.az` contains common type definitions, enums (`FD`, `FLAG`, `FMODE`), and external system call links:

```azlang
// lib.az
enum FD
    Stdin
    Stdout
    Stderr

@link("./sdk/build/exit.o")
op exit(const int val): void

@link("./sdk/build/write.o")
op write(FD fd, const str val, const int size): void
```

---

## 2. Compilation Model for Modules

When an `import` directive is encountered:
1. The parser resolves the target file in the module search path.
2. The imported module AST is parsed and analyzed.
3. Symbol tables are merged or qualified, ensuring all linked `.o` files and symbols are provided to the linker.
