# The Minimalist Doctrine

In classical aerospace and mechanical engineering, the highest reliability is achieved not by complicating safety redundancies, but by **engineering unnecessary valves and moving parts out of the machine entirely**.

AzLang brings this functional minimalist mindset to software engineering:

```text
┌───────────────────────────────────────────────────────────────────────────────────┐
│ AZLANG FIELD DOCTRINE                                                             │
│                                                                                   │
│  [Source Code] ──► [Direct AST Lowering] ──► [2.6 KB Bare-Metal Binary]           │
│                                                                                   │
│  • Self-contained toolchain with zero runtime dependencies.                       │
│  • Executes anywhere, compiles in microseconds, leaves zero operational footprint.│
└───────────────────────────────────────────────────────────────────────────────────┘
```

---

## 1. Field Reliability vs. Heavy Infrastructure

Traditional compiler architectures often assume an infinite logistical tail:
- Background daemons continuously running in memory.
- Gigabytes of backend frameworks (like giant LLVM installation footprints).
- Multi-megabyte runtime libraries and dynamic linker dependencies.

**AzLang rejects this assumption.** 

AzLang is designed as a self-contained, field-deployable compiler:
- **No background daemons**: Compiles directly in one shot.
- **Direct AST Lowering**: AST nodes lower into a lean intermediate representation (QBE / direct IR) without requiring gigabytes of compiler caches.
- **Microsecond compilation**: The compiler executable can be dropped onto an environment, parse code, validate semantics, emit intermediate code, and link a binary in milliseconds.

---

## 2. Kinetic Thrust: Direct Execution Paths

Rather than performing multi-pass combinatorial saturations that bloat compile times, AzLang focuses on **kinetic thrust**:
- Short, deterministic execution paths.
- Lean native assembly generated directly for target architectures.
- High thrust-to-weight ratio in generated code: no dead code bloat, no virtual dispatch tables unless explicitly constructed.

| Metric | AzLang Engineering Model | Heavy Industrial Toolchains |
|---|---|---|
| **Compilation Latency** | Microseconds to milliseconds | Seconds to minutes |
| **Output Binary Size** | Typically 2.0 KB – 3.0 KB | 5 MB – 50 MB |
| **Runtime Dependencies** | None (Direct Kernel Syscalls) | libc, libm, libpthread, dynamic runtimes |
| **Hardware Targets** | Bare metal, microcontrollers, embedded, microservices | Cloud servers, heavy multi-core desktops |

---

## 3. Developer Ergonomics: The Helmet Sight

Soviet fighter aircraft pioneered Helmet-Mounted Cueing Systems (such as the Shchel-3UM) paired with clean, uncluttered analog cockpits. The pilot simply looks at a target and engages; the instruments stay out of the way.

AzLang adopts this philosophy for Developer Experience (DX):
- **Readable indentation-based syntax** akin to Python.
- **Clear explicit boundary typing**: Types are inferred automatically where obvious, and explicitly declared at structural boundaries (`enum`, `object`, parameter signatures).
- The language frontend stays clear and readable while the backend quietly outputs a lethal, high-speed native binary.
