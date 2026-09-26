# Data Types

AzLang provides a focused set of primitive and composite types that map directly to hardware registers and low-level data layouts.

---

## 1. Numeric Types

| Type | Description | Machine Mapping |
|---|---|---|
| `int` | Standard signed integer | 32-bit / 64-bit word (`w` / `l` in QBE) |
| `natural` | Unsigned non-negative integer | Unsigned machine word |
| `bigint` | Large 64-bit integer | 64-bit long (`l`) |
| `tinyint` | Small 8-bit / 16-bit integer | Byte / Half-word (`b` / `h`) |
| `float` | Floating-point number | Single / Double IEEE-754 (`s` / `d`) |

### Example:
```azlang
const int count = 42
const bigint big_offset = 1000000000
const float ratio = 0.75
```

---

## 2. Textual & Character Types

- `char`: A single character literal (`'a'`).
- `str`: String representation. Depending on declaration, string literals are handled either as static immutable string slices (stored in data sections) or dynamic strings:

```azlang
const str greeting = "Hello, world!"
const char initial = 'A'
```

AzLang also supports **template strings** (string interpolation) with backticks:

```azlang
const str name = "AzLang"
const str msg = `Welcome to ${name}`
```

---

## 3. Boolean & Unit Types

- `bool`: Represents `true` or `false`.
- `void`: Represents an empty or absent return value from an operation.
- `any`: Universal type placeholder for dynamic dispatch or heterogeneous buffers.

---

## 4. Compound Types

- `list<T>`: Linear collection of elements of type `T`.
```azlang
const list<int> numbers = [1, 2, 3, 4]
```
- User-defined types (`enum`, `object`, `type`).
