use std::io::{Read, Seek as SeekTrait, Write};
use std::{fs, io};
use vfs::{File, Seek, VfsResult};

pub(crate) struct LocalFile(pub(crate) fs::File);

impl File for LocalFile {
    fn read(&mut self, buffer: &mut [u8]) -> VfsResult<u64> {
        Ok(self
            .0
            .read(buffer)
            .map(|value| value as u64)
            .map_err(|e| e.to_string())?)
    }

    fn write(&mut self, buffer: &[u8]) -> VfsResult<u64> {
        Ok(self
            .0
            .write(buffer)
            .map(|value| value as u64)
            .map_err(|e| e.to_string())?)
    }

    fn seek(&mut self, from: Seek) -> VfsResult<u64> {
        Ok(self
            .0
            .seek(match from {
                Seek::Start(offset) => io::SeekFrom::Start(offset),
                Seek::End(offset) => io::SeekFrom::End(offset),
                Seek::Current(offset) => io::SeekFrom::Current(offset),
            })
            .map_err(|e| e.to_string())?)
    }
}
