# Installation

AzLang ships as a single compiler binary. There is no runtime to install, no VM, and no LLVM-sized toolchain — the only external tool the compiler invokes is **fasm**.

---

## Building from Source

```bash
cargo build --release
```

This generates the `azcli` binary executable.

---

## Installing fasm

AzLang emits one `.asm` file per build and hands it to [fasm](https://flatassembler.net/), which assembles and links it into a standalone ELF executable in a single pass.

```bash
# Arch Linux
sudo pacman -S fasm

# Debian / Ubuntu
sudo apt install fasm
```

For other platforms, download fasm from <https://flatassembler.net/> and put it on your `PATH`.

---

## Next Steps

Continue to the [Hello World Walkthrough](./hello_world.md) to compile your first native binary.
