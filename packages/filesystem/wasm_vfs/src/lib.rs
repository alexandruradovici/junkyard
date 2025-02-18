pub use bindings::exports::junkyard_vfs::vfs_plugin::vfs::{
    AbsolutePath, File as FileResource, GuestFile as File, GuestFilesystem as Filesystem, Kind,
    Seek, Stat,
};

pub use bindings::junkyard_vfs::vfs_plugin::vfs_host::create_absolute_path;

#[allow(warnings)]
pub mod bindings;

#[macro_export]
macro_rules! export_vfs {
    ($FS: ty, $F: ty, $f: expr) => {
        use std::marker::PhantomData;
        use bindings::exports::junkyard_vfs::vfs_plugin::vfs::{self, Guest};
        use $crate::bindings;

        pub struct WasmFilesystem<FS: Filesystem + 'static, F: File + 'static>(PhantomData<FS>, PhantomData<F>);

        impl<FS: Filesystem + 'static, F: File + 'static> Guest for WasmFilesystem<FS, F> {
            type File = F;

            type Filesystem = FS;

            fn init() -> Result<vfs::Filesystem, ()> {
                Ok(vfs::Filesystem::new($f))
            }
        }

        type WasmVfsComponent = WasmFilesystem<$FS, $F>;

        bindings::export!(WasmVfsComponent with_types_in bindings);
    }
}
