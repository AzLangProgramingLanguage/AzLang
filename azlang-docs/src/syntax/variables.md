# Variables & Mutability

AzLang enforces explicit distinction between immutable constants and mutable variables.

---

## 1. Constants (`const`)

By default, data should remain immutable. Use the `const` keyword to declare constants:

```azlang
const int max_retries = 5
const str host = "127.0.0.1"
const float pi = 3.14159
```

Once defined, attempting to reassign a `const` identifier will produce a compile-time semantic error during the validation phase.

---

## 2. Mutable Variables (`var`)

When state needs to change over time, declare the variable using `var`:

```azlang
var int counter = 0
counter = counter + 1

var str status = "pending"
status = "ready"
```

---

## 3. Type Inference & Annotations

AzLang supports type inference for basic expressions, while allowing explicit type annotations for maximum clarity:

```azlang
// Explicit type annotation
const int port = 8080

// Mutable variable with type
var int connections = 0
connections = 1
```

For custom structures, enums, and module interfaces, explicit types are required at structural boundaries.
