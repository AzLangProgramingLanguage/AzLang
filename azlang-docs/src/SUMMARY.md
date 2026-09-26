# Summary

- [Introduction](./introduction.md)

# Engineering Philosophy
- [The Minimalist Doctrine](./philosophy/doctrine.md)
- [Zero-VM & Absence of Moving Parts](./philosophy/zero_vm.md)

# Getting Started
- [Installation](./getting-started/installation.md)
- [Hello World Walkthrough](./getting-started/hello_world.md)

# Language Reference & Real Syntax
- [Variables & Mutability](./syntax/variables.md)
- [Data Types](./syntax/types.md)
- [Enums](./syntax/enums.md)
- [Operations & Expressions](./syntax/operations.md)
- [Control Flow](./syntax/control_flow.md)
- [Operations & Functions (`op`)](./syntax/functions.md)
- [Modules & Imports](./syntax/modules.md)

# Low-Level Interop & SDK
- [Direct ABI Linking (`@link`)](./sdk/link.md)
- [Kernel Syscalls & Bare-Metal Stubs](./sdk/syscalls.md)
- [Standard SDK Library (`lib.az` & `sample.az`)](./sdk/standard_library.md)

# Architecture & Compiler Pipeline
- [Pipeline: Tokenizer to AST](./internals/frontend.md)
- [Semantic Validation & Symbol Tables](./internals/validator.md)
- [QBE Intermediate Representation](./internals/qbe.md)
- [Direct Assembly & Linking](./internals/assembly_linking.md)
