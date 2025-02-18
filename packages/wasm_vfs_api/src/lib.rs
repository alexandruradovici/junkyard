wasmtime::component::bindgen!({
    async: false,
    path: "wit/vfs.wit",
    additional_derives: [
        Eq,
        PartialEq,
        Ord,
        PartialOrd
    ]
});

pub use exports::junkyard_vfs::vfs_plugin::vfs::{Kind, Seek, Stat};
