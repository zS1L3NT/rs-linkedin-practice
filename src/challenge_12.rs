use std::{
    fs::{metadata, File},
    path,
};

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        File::open(&self).is_ok()
    }

    fn is_writeable(&self) -> bool {
        if let Ok(metadata) = metadata(self) {
            !metadata.permissions().readonly()
        } else {
            false
        }
    }

    fn exists(&self) -> bool {
        self.exists()
    }
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(false);
    fs::set_permissions(f.path(), perms).unwrap();
    fs::remove_file(f.path()).unwrap();
}
