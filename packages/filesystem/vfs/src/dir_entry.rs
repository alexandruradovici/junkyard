use crate::AbsolutePath;

#[derive(PartialEq, Debug)]
pub struct DirEntry {
    path: AbsolutePath,
}

impl DirEntry {
    pub fn new(path: AbsolutePath) -> DirEntry {
        DirEntry { path }
    }

    pub fn name(&self) -> &str {
        self.path.name()
    }

    pub fn path(&self) -> &AbsolutePath {
        &self.path
    }
}
