# AzLang

> A minimal, powerful and readable programming language that blends **Python's simplicity**, **Rust's performance**, and **TypeScript's type-safety**.

<p align="center">
  <img src="https://img.shields.io/badge/build-passing-blue.svg" alt="build">
  <img src="https://img.shields.io/badge/license-MIT-red.svg" alt="license">
  <img src="https://img.shields.io/badge/language-AzLang-green.svg" alt="language">
</p>

---

## Our Mission

**AzLang** provides an approachable, understandable and performant coding experience for everyone — from beginners to professional developers.

AzLang is a **compiled** language. The source is parsed into an AST, semantically validated, and then lowered through a dedicated backend. There is no runtime interpreter for `.az` scripts.

---

## Features

- **Natural syntax** — Code reads as clearly as a human language
- **Type-Safety** — Type checking is performed automatically, and optional explicit annotations are supported where needed
- **Static analysis** — A built-in type analysis pipeline validates programs before code generation
- **Compiler-based** — Source is compiled to a native executable; no runtime interpreter

---

## Type System

AzLang's type system relies on automatic type inference. Annotations are optional, though they are required for certain cases such as enums and objects.

```
enum, object and a few specific constructs require an explicit type.
```

---

## Community and Contributions

This project is open source. Any help or idea is valuable:

- New syntax proposals
- Bug reports
- Documentation support
- Code contributions (pull requests are welcome!)

See [CONTRIBUTING.md](CONTRIBUTING.md) for architecture and contribution guidelines.

---

## Roadmap

- Syntax design
- AST and parser
- Type analysis
- Backend / code generation
- Optimization
- Standard library
- Web IDE and playground
- Official documentation and tutorials

---

## Syntax Overview

```azlang
const int a = 5
a = 2

const str b = "Hi"

func add(a: int, b: int): int
    return a + b

print(add(1, 2))
```

---

## Building

```bash
cargo build --release
```

## Dependencies

- [QBE](https://c9x.me/compile/) — afterburner backend / intermediate representation compiler
- **Linux** — `binutils` (assembler) and `ld.lld` (linker)
- **Windows** — an assembler (e.g. MASM or NASM) and `lld.link` (linker)