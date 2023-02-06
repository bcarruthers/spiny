use std::{fs::File, io::*, path::*};
use thiserror::Error;
use zip::{read::ZipFile, ZipArchive};

fn visit_dirs(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files)?;
            } else {
                files.push(entry.path());
            }
        }
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum FileArchiveError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error("directory not found")]
    NotFound,
}

pub type FileArchiveResult<T> = core::result::Result<T, FileArchiveError>;

pub enum FileArchive {
    ZipFile(ZipArchive<File>),
    ZipMemory(ZipArchive<Cursor<Vec<u8>>>),
    Dir(PathBuf),
}

impl FileArchive {
    pub fn from_memory(cursor: Cursor<Vec<u8>>) -> FileArchiveResult<Self> {
        let archive = zip::ZipArchive::new(cursor).map_err(FileArchiveError::Zip)?;
        Ok(FileArchive::ZipMemory(archive))
    }

    pub fn from_path(path: &Path) -> FileArchiveResult<Self> {
        if path.is_dir() {
            Ok(Self::Dir(path.to_path_buf()))
        } else {
            let archive = {
                let zipfile = std::fs::File::open(path)?;
                zip::ZipArchive::new(zipfile)
            }
            .map_err(FileArchiveError::Zip)?;
            Ok(FileArchive::ZipFile(archive))
        }
    }

    pub fn from_any_path(paths: &[PathBuf]) -> FileArchiveResult<Self> {
        for path in paths.iter() {
            if path.exists() {
                log::info!("Path {:?} found", path.to_string_lossy());
                return Self::from_path(path);
            } else {
                log::info!("Path {:?} not found", path.to_string_lossy());
            }
        }
        FileArchiveResult::Err(FileArchiveError::NotFound)
    }

    fn files_in_zip<R: Read + Seek>(archive: &ZipArchive<R>, path: &Path) -> Vec<PathBuf> {
        let cow = path.to_string_lossy();
        let prefix = cow.as_ref();
        archive
            .file_names()
            .filter(|name| name.starts_with(prefix))
            .map(|str| Path::new(str).to_path_buf())
            // Don't use is_file() since that checks file system
            // We're looking for directories like 'textures/'
            .filter(|path| !path.as_os_str().to_string_lossy().ends_with("/"))
            .collect()
    }

    pub fn files_in(&self, dir: &Path) -> FileArchiveResult<Vec<PathBuf>> {
        match self {
            FileArchive::ZipFile(archive) => Ok(Self::files_in_zip(archive, dir)),
            FileArchive::ZipMemory(archive) => Ok(Self::files_in_zip(archive, dir)),
            FileArchive::Dir(root) => {
                let base_path = root.join(dir);
                let mut files = Vec::new();
                visit_dirs(&base_path, &mut files).map_err(FileArchiveError::Io)?;
                let files = files
                    .into_iter()
                    .map(|path| path.as_path().strip_prefix(root).unwrap().to_path_buf())
                    .collect::<Vec<_>>();
                Ok(files)
            }
        }
    }
}

pub enum FileArchiveReader<'a> {
    Zip(ZipFile<'a>),
    File(File),
}

impl<'a> Read for FileArchiveReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Self::Zip(file) => file.read(buf),
            Self::File(file) => file.read(buf),
        }
    }
}

impl FileArchive {
    fn open_zip<'a, R: Read + std::io::Seek>(
        archive: &'a mut ZipArchive<R>,
        path: &Path,
    ) -> FileArchiveResult<FileArchiveReader<'a>> {
        let name = path.to_string_lossy();
        let file = archive.by_name(&name).map_err(FileArchiveError::Zip)?;
        let result = FileArchiveReader::Zip(file);
        Ok(result)
    }

    pub fn open<'a>(&'a mut self, path: &Path) -> FileArchiveResult<FileArchiveReader<'a>> {
        match self {
            FileArchive::ZipFile(archive) => Self::open_zip(archive, path),
            FileArchive::ZipMemory(archive) => Self::open_zip(archive, path),
            FileArchive::Dir(dir) => {
                let fname = dir.join(path);
                let file = std::fs::File::open(&fname).map_err(FileArchiveError::Io)?;
                Ok(FileArchiveReader::File(file))
            }
        }
    }

    pub fn read_string(&mut self, path: &Path) -> FileArchiveResult<String> {
        let mut str = String::new();
        self.open(path)?.read_to_string(&mut str)?;
        Ok(str)
    }
}
