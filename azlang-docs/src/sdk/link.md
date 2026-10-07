# Direct ABI Linking (`@link`)

One of AzLang's most powerful capabilities is seamless, zero-overhead interoperability with native assembly code — no C runtime, no FFI wrappers, no dynamic loading.

---

## 1. The `@link` Directive

To call native code, point `@link` at a **fasm assembly source** and declare the operation's signature:

```azlang
@link("<path-to-fasm-source>")
op <function_name>(<parameters>): <return_type>
```

### Example:
```azlang
@link("./sdk/src/write.s")
op write(FD fd, const str val, const int size): void

@link("./sdk/src/exit.s")
op exit(const int val): void
```

---

## 2. Compiler Pipeline Integration

When the AzLang compiler encounters an `@link` directive:

1. **Parser**: The AST node `Statement::ExternalFunctionDef` records:
   - `name`: Target symbol name (e.g. `write`).
   - `library`: Path to the assembly source (e.g. `./sdk/src/write.s`).
   - `params` & `return_typ`: Checked against call sites.
2. **Validator**: Registers `write` as a valid function and adds `("write", "./sdk/src/write.s")` to the compilation's `link_files` registry.
3. **Transpiler**: When `write` is actually *called*, the compiler canonicalizes the path and emits it into the generated assembly:

```asm
include "/abs/path/to/sdk/src/write.s"
```

   Sources of operations that are never invoked are skipped, and duplicate paths are included only once — unused stubs never reach the binary.
4. **fasm**: The included source and the generated code live in the same translation unit, so `call write` resolves statically at assembly time.

No dynamic library lookups at runtime, no `dlopen` overhead: the code is assembled directly into the binary image in a single `fasm main.asm main` pass.
