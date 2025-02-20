---
layout: section
---

<v-click>

# WASM Component Model
WASI Preview 2

</v-click>

Is there any way to share data with WASM modules in a standard way?

---
---
# Writing WASI was a challenge
How do we share the data types in between modules?

<v-clicks>

- How can we use/link multiple modules together?
- How do we share memory?
- How do we make sure that all modules use the same data representation?

</v-clicks>

---
---
# WebAssembly Component Model

<v-clicks>

- defines an IDL language - *WIT*
- defines a set of data types that can be exchanged
- defines a set of linking rules

</v-clicks>

<br>
<br>
<br>

<v-click>

[Write once in any language, link it to any other components and run it everywhere.]{style="color: green"}

</v-click>

<v-click>

*Has the potential to become de de-facto way of distributing applications.*

</v-click>

---
---
# WASI Preview 2
Running WASM Components Model

- We can use this in Rust to build safe plugins!
- [Sandboxed]{style="color: green"} shared Rust-like libraries

Rust/LLVM (closely) provides multiple targets for WASM Core

| Target | What can we do |
|-|-|
| *wasm32-wasip2* | access platform functions in a [controlled]{style="color: orange"} manner |
