# Control Flow

AzLang utilizes significant whitespace and indentation (4 spaces per level) to organize execution blocks cleanly, avoiding noisy curly braces or verbose block delimiters.

---

## 1. Conditional Branching (`if`, `elif`, `else`)

Conditional statements evaluate boolean expressions to branch execution paths:

```azlang
const int value = 42

if value == 0
    write(Stdout, "Value is zero\n", 14)
elif value > 0
    write(Stdout, "Value is positive\n", 18)
else
    write(Stdout, "Value is negative\n", 18)
```

---

## 2. While Loops (`while`)

The `while` loop continuously executes an indented statement block as long as the condition remains `true`:

```azlang
var int i = 0

while i < 10
    i = i + 1
```

---

## 3. Infinite Loops (`loop`)

For continuous polling, kernel event pumps, or server request loops, use the bare `loop` construct:

```azlang
loop
    // execution body
    if should_stop
        break
```

---

## 4. Iteration (`for .. in`)

AzLang supports iterating through lists and collections:

```azlang
const list<int> items = [10, 20, 30]

for items in item
    // use item
```

---

## 5. Loop Control Statements

- `break`: Terminate the loop immediately.
- `continue`: Skip the rest of the current iteration and proceed to the next cycle.
