wasmtime::component::bindgen!({
    async: false,
    path: "wit/vfs.wit",
    additional_derives: [
        Eq,
        PartialEq,
        Ord,
        PartialOrd
    ],
    with: {
        "junkyard-vfs:vfs-plugin/vfs-host/absolute-path": AbsolutePath
    }
});

pub use exports::junkyard_vfs::vfs_plugin::vfs::{Kind, Seek, Stat};

mod path;

// local resources
pub use path::AbsolutePath;