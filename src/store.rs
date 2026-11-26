use std::ffi::OsString;
use std::fs::{self, File, OpenOptions, Permissions};
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};

use tempfile::NamedTempFile;

use crate::error::Error;
use crate::model::MappingsFile;

pub fn load(path: &Path) -> Result<MappingsFile, Error> {
    let file: File = File::open(path)?;
    return Ok(serde_yaml_ng::from_reader(BufReader::new(file))?);
}

/// Replace or create a document atomically.
pub fn save(path: &Path, document: &MappingsFile) -> Result<(), Error> {
    return with_lock(path, |path| {
        return save_locked(path, document);
    });
}

/// Hold the same lock from before reading through the final rename.
pub fn update(
    path: &Path,
    operation: impl FnOnce(&mut MappingsFile) -> Result<(), Error>,
) -> Result<(), Error> {
    return with_lock(path, |path| {
        let mut document: MappingsFile = load(path)?;
        operation(&mut document)?;
        return save_locked(path, &document);
    });
}

fn with_lock(path: &Path, operation: impl FnOnce(&Path) -> Result<(), Error>) -> Result<(), Error> {
    let path: PathBuf = match path.canonicalize() {
        Ok(path) => path,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            // A dangling symlink is an error rather than a new regular file.
            match path.symlink_metadata() {
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
                Ok(_) => return Err(error.into()),
            }
            let name: &std::ffi::OsStr = path
                .file_name()
                .ok_or_else(|| Error::Invalid("expected a mappings file path".to_owned()))?;
            let parent: &Path = path
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .unwrap_or_else(|| Path::new("."));
            parent.canonicalize()?.join(name)
        }
        Err(error) => return Err(error.into()),
    };

    // Lock a stable inode because every save replaces the document's inode.
    // Keep the sidecar so all processes keep locking the same inode.
    let mut lock_path: OsString = path.as_os_str().to_owned();
    lock_path.push(".lock");
    let lock: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)?;
    lock.lock()?;
    return operation(&path);
}

fn save_locked(path: &Path, document: &MappingsFile) -> Result<(), Error> {
    let permissions: Option<Permissions> = match fs::metadata(path) {
        Ok(metadata) => Some(metadata.permissions()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    let parent: &Path = path
        .parent()
        .ok_or_else(|| Error::Invalid("expected a mappings file path".to_owned()))?;
    let mut file: NamedTempFile = NamedTempFile::new_in(parent)?;
    serde_yaml_ng::to_writer(&mut file, document)?;
    file.flush()?;
    if let Some(permissions) = permissions {
        file.as_file().set_permissions(permissions)?;
    }
    file.persist(path).map_err(|error| error.error)?;
    return Ok(());
}

#[cfg(test)]
mod tests {
    use std::sync::Barrier;
    use std::thread;

    use super::*;
    use crate::ops::{self, CreateComponent};

    #[test]
    fn preserves_each_concurrent_update() {
        const WRITERS: usize = 12;
        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let path: PathBuf = directory.path().join("mappings.yaml");
        save(&path, &MappingsFile::default()).unwrap();
        let barrier: Barrier = Barrier::new(WRITERS);
        thread::scope(|scope| {
            for index in 0..WRITERS {
                let path: &Path = &path;
                let barrier: &Barrier = &barrier;
                scope.spawn(move || {
                    let input: CreateComponent = CreateComponent {
                        component_name: format!("Gateway {index}"),
                        component_type: "service".to_owned(),
                        description: "Example gateway".to_owned(),
                        purpose: "Filter requests".to_owned(),
                    };
                    barrier.wait();
                    update(path, |document| {
                        return ops::create_component(&mut document.components, &input);
                    })
                    .unwrap();
                });
            }
        });
        let document: MappingsFile = load(&path).unwrap();
        assert_eq!(document.components.len(), WRITERS);
    }

    #[cfg(unix)]
    #[test]
    fn preserves_permissions_on_replace() {
        use std::os::unix::fs::PermissionsExt;

        let directory: tempfile::TempDir = tempfile::tempdir().unwrap();
        let path: PathBuf = directory.path().join("mappings.yaml");
        let document: MappingsFile = MappingsFile::default();
        save(&path, &document).unwrap();
        fs::set_permissions(&path, Permissions::from_mode(0o640)).unwrap();
        save(&path, &document).unwrap();
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o640
        );
    }
}
