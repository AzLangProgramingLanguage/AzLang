# Semantic Validation & Symbol Resolution

Before any code generation takes place, AzLang programs pass through semantic validation.

---

## 1. Type and Symbol Verification

During semantic validation, the compiler analyzes program declarations to verify:

- **Type Safety**: Variable assignments, arithmetic expressions, and function arguments match expected types.
- **Scope Management**: Variables, constants, and functions are validated within their lexical scopes.
- **Link Registry**: External function declarations (`@link`) are recorded and prepared for the linker phase.
- **Unused & Mutation Tracking**: Identifies immutable violations and unused bindings at compile-time.

---

## 2. Compile-Time Constant & Enum Resolution

AzLang resolves static constants and enumerations during validation:

- **Enum Inlining**: Enum variants are assigned zero-based sequential numbers and stored directly in the compiler's constant table.
- **Zero Runtime Memory**: Any reference to an enum variant is converted directly into an immediate value before code generation. No lookup tables, dynamic allocations, or wrapper structures exist at runtime.
