# Standard SDK Library

The official AzLang SDK resides in the `sdk/` directory (tracked as a git submodule). It acts as the reference standard library implementation demonstrating how systems programming is conducted in pure AzLang.

---

## 1. Structure of `sdk/`

```text
sdk/
├── src/
│   ├── lib.az        # Common types, enums, and @link declarations
│   ├── write.s       # Linux sys_write assembly stub (fasm syntax)
│   ├── exit.s        # Linux sys_exit assembly stub (fasm syntax)
│   └── fopen.s       # Linux sys_open assembly stub (fasm syntax)
└── README.md
```

There is no build step: the `.s` files are fasm assembly *sources* referenced by `@link` and included directly into the compiler's generated assembly.

---

## 2. Standard Types in `lib.az`

```azlang
enum FD
    Stdin
    Stdout
    Stderr

enum FLAG
    ReadOnly
    Write

enum FMODE
    NoCreate
    Create

@link("./sdk/src/exit.s")
op exit(const int val): void

@link("./sdk/src/write.s")
op write(FD fd, const str val, const int size): void

@link("./sdk/src/fopen.s")
op fopen(const str val, FLAG flag, FMODE mode): File
```

---

## 3. Sample Application

```azlang
// hello_world.az
import lib

write(Stdout, "Hello World\n", 12)
```

Build it with a single command:

```bash
azcli build hello_world.az
./hello_world
```
