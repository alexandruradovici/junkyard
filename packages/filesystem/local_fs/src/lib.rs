use file::LocalFile;
use std::fs;
use vfs::{AbsolutePath, DirEntry, File, Kind, OpenOptions, Stat, Vfs, VfsResult};

mod file;

pub struct LocalFileSystem {}

impl Vfs for LocalFileSystem {
    // Files
    fn open(&self, path: &AbsolutePath, open_options: OpenOptions) -> VfsResult<Box<dyn File>> {
        let f = fs::File::options()
            .append(open_options.append)
            .create(open_options.create)
            .read(open_options.read)
            .truncate(open_options.truncate)
            .write(open_options.write)
            .open(path.as_str())?;
        Ok(Box::new(LocalFile(f)))
    }
    fn unlink(&self, _path: &AbsolutePath) -> VfsResult<()> {
        todo!()
    }
    fn stat(&self, path: &AbsolutePath) -> VfsResult<Stat> {
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

    // Folders
    fn read_dir(&self, path: &AbsolutePath) -> VfsResult<Vec<DirEntry>> {
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
    fn create_dir(&self, path: &AbsolutePath) -> VfsResult<()> {
        Ok(fs::create_dir(path.as_str())?)
    }
    fn create_dir_all(&self, path: &AbsolutePath) -> VfsResult<()> {
        Ok(fs::create_dir_all(path.as_str())?)
    }

    // All
    fn rename(&self, from: &AbsolutePath, to: &AbsolutePath) -> VfsResult<()> {
        Ok(fs::rename(from.as_str(), to.as_str())?)
    }
}

#[cfg(test)]
mod tests {
    use crate::LocalFileSystem;
    use std::{
        env::{self, temp_dir},
        fs::{self, Permissions},
        io::Result as VfsResult,
        os::unix::fs::PermissionsExt,
    };

    use vfs::{DirEntry, Vfs};

    fn dir_to_vec(mut dir: fs::ReadDir) -> VfsResult<Vec<fs::DirEntry>> {
        let mut vec = vec![];
        while let Some(entry) = dir.next() {
            let entry = entry?;
            if entry.file_name() != "." {
                vec.push(entry);
            }
        }
        Ok(vec)
    }

    fn read_folder(folder: impl AsRef<str>) -> (Vec<DirEntry>, Vec<fs::DirEntry>) {
        let local_vfs = LocalFileSystem {};
        let path = folder.as_ref().into();

        let entries = local_vfs.read_dir(&path).unwrap();

        let local_files = dir_to_vec(fs::read_dir(folder.as_ref()).unwrap()).unwrap();

        (entries, local_files)
    }

    fn test_read_folder(folder: impl AsRef<str>) {
        let (mut entries, mut local_files) = read_folder(folder);

        entries.sort_by(|e1, e2| e1.name().cmp(e2.name()));
        local_files.sort_by(|e1, e2| e1.file_name().cmp(&e2.file_name()));

        assert_eq!(entries.len(), local_files.len());

        entries
            .into_iter()
            .zip(local_files.into_iter())
            .for_each(|(entry, local_entry)| assert_eq!(entry.name(), local_entry.file_name()));
    }

    #[test]
    fn list_current_folder() {
        let folder = env::current_dir()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();

        test_read_folder(folder);
    }

    #[test]
    fn list_root_folder() {
        test_read_folder("/");
    }

    #[test]
    fn list_tmp_folder() {
        test_read_folder("/tmp");
    }

    #[test]
    fn list_dev_folder() {
        test_read_folder("/dev");
    }

    #[test]
    fn list_etc_folder() {
        test_read_folder("/etc");
    }

    #[test]
    fn list_no_folder() {
        // let local_vfs = LocalFileSystem {};
        // let path = "/is_not_folder".into();

        // assert_eq!(
        //     local_vfs.read_dir(&path).err().unwrap().kind(),
        //     ErrorKind::NotFound
        // );
    }

    #[test]
    fn no_current_root() {
        let local_vfs = LocalFileSystem {};
        let path = "/".into();

        let entries = local_vfs.read_dir(&path).unwrap();
        assert_eq!(
            entries.into_iter().find(|folder| folder.name() == "."),
            None
        );
    }

    #[test]
    fn no_current_folder() {
        let local_vfs = LocalFileSystem {};
        let path = "/tmp".into();

        let entries = local_vfs.read_dir(&path).unwrap();
        assert_eq!(
            entries.into_iter().find(|folder| folder.name() == "."),
            None
        );
    }

    #[test]
    fn no_parent_root() {
        let local_vfs = LocalFileSystem {};
        let path = "/".into();

        let entries = local_vfs.read_dir(&path).unwrap();
        assert_eq!(
            entries.into_iter().find(|folder| { folder.name() == ".." }),
            None
        );
    }

    #[test]
    fn no_parent_folder() {
        let local_vfs = LocalFileSystem {};
        let path = "/tmp".into();

        let entries = local_vfs.read_dir(&path).unwrap();
        assert_eq!(
            entries.into_iter().find(|folder| { folder.name() == ".." }),
            None
        );
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn list_permission_denied_folder() {
        let mut folder = temp_dir();
        folder.push("no_access");
        let _local_vfs = LocalFileSystem {};
        fs::create_dir_all(&folder).unwrap();
        let permissions = Permissions::from_mode(0o000);
        fs::set_permissions(&folder, permissions).unwrap();
        // let path = folder.as_os_str().to_str().unwrap().into();

        // let error = local_vfs.read_dir(&path).err().unwrap().kind();

        fs::remove_dir(&folder).unwrap();

        // assert_eq!(error, ErrorKind::PermissionDenied);
    }
}
