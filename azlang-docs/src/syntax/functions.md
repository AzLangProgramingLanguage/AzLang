# Operations & Functions (`op`)

In AzLang's real SDK syntax, callable procedures and functions are declared using the **`op`** keyword. This aligns with AzLang's low-level systems nature: procedures are atomic machine operations.

---

## 1. Declaring Internal Operations

An operation defines parameters, parameter types, a return type, and an indented block:

```azlang
op add(const int a, const int b): int
    return a + b

op multiply(var int x, const int factor): int
    return x * factor
```

### Mutability in Parameters
- `const <Type> <name>`: Value parameter passed immutably.
- `var <Type> <name>`: Parameter treated as a mutable pointer reference.

---

## 2. Returning Void

When an operation does not yield a value (e.g. system exit or logging), its return type is specified as `void`:

```azlang
op log_message(const str msg, const int len): void
    write(Stdout, msg, len)
```

---

## 3. External Operations (`@link` + `op`)

To declare an interface to an external assembly source, place the `@link` directive above the `op` signature:

```azlang
@link("./sdk/src/write.s")
op write(FD fd, const str val, const int size): void

@link("./sdk/src/exit.s")
op exit(const int val): void

@link("./sdk/src/fopen.s")
op fopen(const str val, FLAG flag, FMODE mode): File
```

When AzLang compiles this file, it registers the external function signature in its symbol table and, for every operation that is actually called, emits an `include` directive pulling the assembly source into the generated `.asm` — fasm then resolves the symbol statically in the same pass.
