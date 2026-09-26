# Operations & Expressions

AzLang supports standard binary and unary operations with strict type checks enforced during compiler validation.

---

## 1. Arithmetic Operators

AzLang supports the standard arithmetic operations:

| Operator | Meaning | Example |
|---|---|---|
| `+` | Addition | `a + b` |
| `-` | Subtraction | `a - b` |
| `*` | Multiplication | `a * b` |
| `/` | Division | `a / b` |
| `%` | Modulo | `a % b` |

### Example:
```azlang
const int a = 10
const int b = 3
const int sum = a + b
const int rem = a % b
```

---

## 2. Comparison Operators

Comparisons evaluate to a `bool` (`true` or `false`):

| Operator | Description |
|---|---|
| `==` | Equal |
| `!=` | Not equal |
| `>` | Greater than |
| `>=` | Greater than or equal |
| `<` | Less than |
| `<=` | Less than or equal |

### Example:
```azlang
const bool is_equal = (a == b)
const bool is_positive = (a > 0)
```

---

## 3. Logical Operators

AzLang provides natural keywords as well as standard symbol alternatives:
- `and` / `&&` : Logical AND
- `or` / `||` : Logical OR
- `!` : Logical NOT

```azlang
if a > 0 and b > 0
    // block
```
