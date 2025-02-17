

use std::io::SeekFrom;

pub type VfsResult<T> = anyhow::Result<T>;

mod dir_entry;
mod path;
mod stat;

pub use dir_entry::DirEntry;
pub use path::AbsolutePath;
pub use stat::{Kind, Stat};

#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub struct OpenOptions {
    pub create: bool,
    pub read: bool,
    pub write: bool,
    pub truncate: bool,
    pub append: bool,
}

impl OpenOptions {
    pub fn create() -> Self {
        OpenOptions {
            create: true,
            read: false,
            write: true,
            truncate: false,
            append: false,
        }
    }

    pub fn read() -> Self {
        OpenOptions {
            create: false,
            read: true,
            write: false,
            truncate: false,
            append: false,
        }
    }

    pub fn truncate(create: bool) -> Self {
        OpenOptions {
            create,
            read: false,
            write: true,
            truncate: true,
            append: false,
        }
    }

    pub fn append(create: bool) -> Self {
        OpenOptions {
            create,
            read: false,
            write: true,
            truncate: false,
            append: true,
        }
    }

    pub fn read_write(create: bool) -> Self {
        OpenOptions {
            create,
            read: true,
            write: true,
            truncate: false,
            append: false,
        }
    }
}

pub trait File {
    fn read(&mut self, buffer: &mut [u8]) -> VfsResult<u64>;
    fn write(&mut self, buffer: &[u8]) -> VfsResult<u64>;
    fn seek(&mut self, from: SeekFrom) -> VfsResult<u64>;
}

pub trait Vfs: Send + Sync {
    // Files
    fn open(&self, path: &AbsolutePath, open_options: OpenOptions) -> VfsResult<Box<dyn File>>;
    fn unlink(&self, path: &AbsolutePath) -> VfsResult<()>;
    fn stat(&self, path: &AbsolutePath) -> VfsResult<Stat>;

    // Folders
    fn read_dir(&self, path: &AbsolutePath) -> VfsResult<Vec<DirEntry>>;
    fn create_dir(&self, path: &AbsolutePath) -> VfsResult<()>;
    fn create_dir_all(&self, path: &AbsolutePath) -> VfsResult<()>;

    // All
    fn rename(&self, from: &AbsolutePath, to: &AbsolutePath) -> VfsResult<()>;
}

#[cfg(test)]
mod test {
    use crate::Vfs;

    #[test]
    fn object_safe() {
        #[allow(unused)]
        fn dispatch(_vfs: &dyn Vfs) {}
    }
}
