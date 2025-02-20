---
# You can also start simply with 'default'
theme: seriph
aspectRatio: 16/9
# random image from a curated Unsplash collection by Anthony
# like them? see https://unsplash.com/collections/94734566/slidev
background: ./auto.png
# some information about your slides (markdown enabled)
title: Building Software Extensions in Rust using WebAssembly Components
info: |
  ## Slidev Starter Template
  Presentation slides for developers.

  Learn more at [Sli.dev](https://sli.dev)
# apply unocss classes to the current slide
class: text-center
# https://sli.dev/features/drawing
drawings:
  persist: false
# slide transition: https://sli.dev/guide/animations.html#slide-transitions
colorSchema: dark
# enable MDC Syntax: https://sli.dev/features/mdc
mdc: true
highlighter: shiki
lineNumbers: true
---

# Building Software Extensions in Rust using WebAssembly Components

Alexandru Radovici

---
src: ./plugin.md
---

---
src: ./wasm_core.md
---

---
src: ./wasm_components.md
---

---
src: ./elements.md
---

---
src: ./junkyard.md
---


---
---
# Lessons learned

<v-clicks>

- tools are changing a lot ... documentation is sometimes outdated
- working with `wasmtime` is not very straight forward
- make sure you cache your plugins, `wasmtime` instantiates (compiles) them AOT
- WASM Components are not (yet) native to Rust (not event for target `wasm32-wasip2`)
- the tools do not generate symmetric traits, you have to write your own host traits
- resource management is manual ... its easy to forget to drop a resource

</v-clicks>
