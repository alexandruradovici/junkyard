---
layout: section
---
# Plugins
extensions to existing applications

---
---
# Requirements

<v-clicks>

- should be able to *write them in different languages* (Rust, Python, TypeScript)
- should *run fast*
- should *run sandboxed* (not able to compromise the app)

</v-clicks>

<br>

<v-click>

[the provider might not want to share the source code]{style="color: orange"}

</v-click>

<br>

<v-click>

everything boils down to safely running and interacting with 3rd party code

</v-click>

---
---
# Solutions

| Solution | Languages | Speed | Sandbox |
|-|-|-|-|
| **C-ABI** | any compiled language | ✅ | ❌ |
| *Script Language* | Lua / Embedded JS Engine | ❌ | ✅ |
| Rust Modules? | Rust | ✅ | [borrow checker]{style="color: orange"} |

<br>

### Why not Rust modules?
- Rust has no stable ABI, it may change from a version to another
- the compiler has to see the whole source code
