use std::fs;

use vfs::{AbsolutePath, DirEntry, Kind, Stat, Vfs};
use wasm_vfs::export_vfs;

struct LocalVfs {}

impl Vfs for LocalVfs {
    fn open(
        &self,
        _path: &AbsolutePath,
        _open_options: vfs::OpenOptions,
    ) -> vfs::VfsResult<Box<dyn vfs::File>> {
        todo!()
    }

    fn unlink(&self, _path: &AbsolutePath) -> vfs::VfsResult<()> {
        todo!()
    }

    fn stat(&self, path: &AbsolutePath) -> vfs::VfsResult<vfs::Stat> {
        let data = fs::metadata(path.as_str())?;
        Ok(Stat {
            kind: if data.is_dir() {
                Kind::Folder
            } else if data.is_symlink() {
                Kind::Link
            } else if data.is_file() {
                Kind::File
            } else {
                Kind::Unknown
            },
            size: data.len(),
        })
    }

    fn read_dir(&self, path: &AbsolutePath) -> vfs::VfsResult<Vec<DirEntry>> {
        let entries = fs::read_dir(path.as_str())?;
        let mut files = vec![];
        for entry in entries {
            if let Ok(entry) = entry {
                files.push(DirEntry::new(AbsolutePath::new(
                    entry.path().to_string_lossy(),
                )));
            }
        }
        Ok(files)
    }

    fn create_dir(&self, _path: &AbsolutePath) -> vfs::VfsResult<()> {
        todo!()
    }

    fn create_dir_all(&self, _path: &AbsolutePath) -> vfs::VfsResult<()> {
        todo!()
    }

    fn rename(&self, _from: &AbsolutePath, _to: &AbsolutePath) -> vfs::VfsResult<()> {
        todo!()
    }
}

export_vfs!(LocalVfs, LocalVfs {});
