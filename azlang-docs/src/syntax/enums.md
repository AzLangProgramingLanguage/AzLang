# Enums

Enums in AzLang provide a zero-cost, type-safe abstraction for representing discrete states, flags, and system descriptors (such as Linux file descriptors, open flags, or socket options).

---

## 1. Syntax

Enums are declared using the `enum` keyword followed by the enum type name and an indented list of variants:

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
```

---

## 2. Zero-Cost Compile-Time Inlining & Memory Representation

In AzLang, **enums do not produce runtime allocation overhead or stack indirection tables**.

### Byte-Level Direct Inlining (Zero Footprint)
During the semantic validation stage, enum variants are registered directly as sequential numeric constants starting from `0`:

- `Stdin  -> 0`
- `Stdout -> 1`
- `Stderr -> 2`

During intermediate code generation, any reference to an enum variant is **directly inlined at compile-time as an immediate byte / literal value** directly into the instruction stream or call site:

```qbe
# Emitted Intermediate Representation
# Notice that no stack allocation or copy instruction is generated.
# The variant is inlined directly as an immediate literal argument:
call $write(l 1, l $str0, l 27)
```

### Key Architectural Benefits:
1. **Zero Stack Allocation**: No temporary register operations needed at runtime.
2. **Byte-Level Memory Density**: Directly fits the exact ABI byte/word requirement of native syscalls with zero runtime indirection.
3. **Optimized Instruction Stream**: Machine code directly emits immediate constants, minimizing executable binary size and instruction cache misses.

---

## 3. Usage with Operations & Native Syscalls

Enums are first-class types during semantic validation. They enforce strict compile-time type safety while passing as zero-cost inlined numbers to ABI operations:

```azlang
enum FD
    Stdin
    Stdout
    Stderr

@link("./sdk/build/write.o")
op write(FD fd, const str val, const int size): void

@link("./sdk/build/exit.o")
op exit(const int val): void

# Stdout is validated as FD type at compile-time, 
# and directly inlined as literal 1 in the emitted binary
write(Stdout, "Writing to standard output...\n", 30)
exit(0)
```
