use std::fs::{self};

use wasm_vfs::{
    create_absolute_path, export_vfs, AbsolutePath, File, FileResource, Filesystem, Kind, Seek,
    Stat,
};

struct LocalFile;

impl File for LocalFile {
    fn read(&self, _data: Vec<u8>) -> Result<u64, String> {
        todo!()
    }

    fn write(&self, _data: Vec<u8>) -> Result<u64, String> {
        todo!()
    }

    fn seek(&self, _s: Seek) -> Result<u64, String> {
        todo!()
    }
}

struct LocalVfs;

impl Filesystem for LocalVfs {
    fn read_dir(&self, path: &AbsolutePath) -> Result<Vec<AbsolutePath>, String> {
        let entries = fs::read_dir(path.path()).map_err(|e| e.to_string())?;
        let mut files = vec![];
        for entry in entries {
            if let Ok(entry) = entry {
                files.push(create_absolute_path(&entry.path().to_string_lossy()));
            }
        }
        Ok(files)
    }

    fn stat(&self, path: &AbsolutePath) -> Result<Stat, String> {
        let data = fs::metadata(path.path()).map_err(|e| e.to_string())?;
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

    fn open(&self, _path: AbsolutePath) -> Result<FileResource, String> {
        todo!()
    }
}

export_vfs!(LocalVfs, LocalFile, LocalVfs);
