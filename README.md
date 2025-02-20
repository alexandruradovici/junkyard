# junkyard

An experimental file manager (commander) that uses WASM Component Model plugins

## crates

| crate | details | host | guest |
|-|-|-|-|
| `junkyard` | the code for the file manager | ✅ | |
| `wasm` | the plugin runner, uses `wasmtime` to load WASM Component plugins | ✅ | |
| `wasm_vfs_api` | exports the automatically generated generated from the WIT file | exports the data structures | exports the data structures and traits |
|  `packages/filesystem/vfs` | exports a Rust trait for interacting wth plugins | ✅ |  |
|  `packages/filesystem/wasm_vfs` | exports the API for plugin implementers |  | ✅ |
|  `packages/filesystem/wasm_local_fs` | a local filesystem plugin |  | ✅ |
|  `packages/filesystem/wasm_local_js_fs` | a local filesystem plugin written in TypeScript |  | ✅ |
|  `packages/filesystem/local_fs` | native implementation of a local filesystem plugin |  | ✅ |
