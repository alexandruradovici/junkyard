---
layout: section
---
# Wasm Interface Type
a standard way of specifying interfaces - inspired from Rust

---
---
# Interfaces
define a set of \{*record*s, *enum*s *varinat*s and *func*tions\} and are the basic element that can be *import*ed or *export*ed

```WIT
interface vfs {
    // ...
}

interface vfs-host {
    // ...
}
```


---
---
# Enum
list of items

```WIT {2-7}
interface vfs {
    enum kind {
        file,
        folder,
        link,
        unknown
    }
}
```

---
---
# Record
struct, no methods

```WIT {2-7}
interface vfs {
    record stat {
        kind: Kind,
        size: u64
    }
}
```

---
---
# Variant
enum with payload

```WIT {2-6}
interface vfs {
    variant seek {
        start(u64),
        current(s64),
        end(s64)
    }
}
```

---
---

# Resources
opaque objects that provide methods

```WIT {2-8}
interface vfs {
    resource absolute-path {
        components: func() -> list<string>;
        is-root: func() -> bool;
        parent: func() -> absolute-path;
        file-name: func() -> string;
        path: func() -> string;
    }
}
```

resources can be *owned* or *borrowed*

```WIT {2-7}
interface vfs {
    resource filesystem {
        read-dir: func(path: borrow<absolute-path>) -> result<list<absolute-path>, string>;
        stat: func(path: borrow<absolute-path>) -> result<stat, string>;

        open: func(path: absolute-path) -> result<file, string>;
    }
}
```

---
---
# World
represent the connections of a component and defines what the component *import*s and *export*s

```WIT
world vfs-plugin {
    // the component can use all the elements in `vfs-host`
    import vfs-host;

    // the host or other components can use all the elements in `vfs`
    export vfs;
}
```

