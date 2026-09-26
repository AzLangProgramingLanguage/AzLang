# Pipeline: Tokenizer to AST

AzLang’s compiler frontend transforms raw human-readable source code into a strongly-typed Abstract Syntax Tree (AST).

---

## 1. Tokenizer

The tokenizer transforms raw source text into a stream of spanned tokens:

- **Indentation Tracking**: Maintains an indentation stack to generate synthetic indent and dedent blocks (based on 4 spaces), giving AzLang its clean, readable syntax.
- **String Templates**: Handles template interpolation (`` `text ${val}` ``) cleanly without dynamic runtime parsing.
- **Accurate Diagnostics**: Tracks line and character offsets for precise compile-time error reporting.

---

## 2. Parser

The parser consumes tokens and builds a strongly-typed Abstract Syntax Tree (AST):

- **Statements**:
  - Variable declarations (`const` / `var`)
  - Operations and procedures (`op`)
  - External system bindings (`@link`)
  - Enumerations (`enum`)
  - Conditional branching blocks (`if`, `elif`, `else`)
  - Loops (`while`, `for`, `loop`)
- **Expressions**:
  - Binary operations (`+`, `-`, `*`, `/`), comparisons, identifiers, literals, function calls, and template expressions.
