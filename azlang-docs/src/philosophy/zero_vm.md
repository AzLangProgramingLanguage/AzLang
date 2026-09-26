# Zero-VM & Absence of Moving Parts

The most resilient system is one without excess complexity. Many modern programming runtimes inject heavy layers between application code and silicon:

1. **Virtual Machines (VM)** with dynamic bytecode interpreters.
2. **Stop-the-world Garbage Collectors (GC)** managing heap pages in the background.
3. **Complex C-runtime Initialization** (`crt0.o`, `crt1.o`, `libc`) running pre-main hooks.
4. **Exception Unwinding Tables** (`.eh_frame`) introducing hidden branches and stack unwinding machinery.

---

## How AzLang Achieves Safety Through Absence

AzLang eliminates every single one of these layers:

### 1. Direct Bare-Metal Entry (`_start`)
AzLang executables do not need `main()` wrapped in standard C-runtime setup. They target `_start` directly. The operating system kernel spawns the process, pushes initial stack frames (arguments, environment pointers), and transfers instruction control directly to AzLang code.

### 2. Zero Heap Leaks by Design
With direct register allocation and stack management, AzLang does not rely on opaque runtime memory arenas. When a program exits, it issues a direct kernel exit syscall (`sys_exit`, `60` on Linux x86_64).

### 3. Predictable State Invariants
Because there are no background scheduler threads, garbage collector pauses, or hidden asynchronous thread pools, program execution is:
- **100% Deterministic**: Timing jitter is near zero.
- **Traceable**: Assembly instructions correspond directly to written operations.
- **Inspectable**: Debugging with tools like `gdb`, `objdump`, or `strace` reveals pure syscalls and predictable stack frames.

```text
[AzLang Application]
        │
        ▼ (direct syscall: rax=1 for write, rax=60 for exit)
[Linux Kernel Ring 0]
```
