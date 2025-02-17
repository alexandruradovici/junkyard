use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use vfs::{File, VfsResult};

pub(crate) struct LocalFile(pub(crate) fs::File);

impl File for LocalFile {
    fn read(&mut self, buffer: &mut [u8]) -> VfsResult<u64> {
        Ok(self.0.read(buffer).map(|value| value as u64)?)
    }

    fn write(&mut self, buffer: &[u8]) -> VfsResult<u64> {
        Ok(self.0.write(buffer).map(|value| value as u64)?)
    }

    fn seek(&mut self, from: SeekFrom) -> VfsResult<u64> {
        Ok(self.0.seek(from)?)
    }
}
