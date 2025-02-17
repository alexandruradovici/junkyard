use bindings::exports::junkyard_vfs::wasm_local_fs::vfs::{GuestFilesystem, Kind, Stat};
use vfs::{AbsolutePath, Vfs};

#[allow(warnings)]
pub mod bindings;

#[macro_export]
macro_rules! export_vfs {
    ($T: ty, $f: expr) => {
        use std::marker::PhantomData;
        use bindings::exports::junkyard_vfs::wasm_local_fs::vfs::{Filesystem, Guest, GuestFilesystem};
        use $crate::{WasmFilesystem, bindings};

        pub struct Component<V: Vfs + 'static>(PhantomData<V>);

        impl Guest for Component<$T> {
            type Filesystem = WasmFilesystem<$T>;

            fn init() -> Result<Filesystem, ()> {
                Ok(Filesystem::new(WasmFilesystem::new($f)))
            }
        }

        type WasmVfsComponent = Component<$T>;

        bindings::export!(WasmVfsComponent with_types_in bindings);
    }
}

impl From<vfs::Stat> for Stat {
    fn from(value: vfs::Stat) -> Self {
        Stat {
            kind: value.kind.into(),
            size: value.size,
        }
    }
}

impl From<vfs::Kind> for Kind {
    fn from(value: vfs::Kind) -> Self {
        match value {
            vfs::Kind::File => Kind::File,
            vfs::Kind::Folder => Kind::Folder,
            vfs::Kind::Link => Kind::Link,
            vfs::Kind::Unknown => Kind::Unknown,
            _ => Kind::Unknown,
        }
    }
}

pub struct WasmFilesystem<V: Vfs + 'static>(V);

impl<V: Vfs + 'static> WasmFilesystem<V> {
    pub fn new(vfs: V) -> WasmFilesystem<V> {
        WasmFilesystem(vfs)
    }
}

impl<V: Vfs + 'static> GuestFilesystem for WasmFilesystem<V> {
    fn read_dir(&self, path: String) -> Result<Vec<String>, String> {
        Ok(self
            .0
            .read_dir(&AbsolutePath::from(path))
            .map_err(|err| err.to_string())?
            .iter()
            .map(|entry| entry.path().as_str().to_string())
            .collect())
    }

    fn stat(&self, path: String) -> Result<Stat, String> {
        self.0
            .stat(&AbsolutePath::new(path))
            .map_err(|e| e.to_string())
            .map(|s| s.into())
    }
}
