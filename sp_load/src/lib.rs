#![forbid(unsafe_code)]

pub mod pack;

use sp_asset::archive::*;
use std::{io::Cursor, path::PathBuf};

fn open_embedded() -> FileArchiveResult<FileArchive> {
    let bytes = include_bytes!("../assets.zip");
    log::info!("Reading embedded assets ({} bytes)", bytes.len());
    let cursor = Cursor::new(bytes.to_vec());
    FileArchive::from_memory(cursor)
}

pub fn open_assets(asset_paths: &[PathBuf]) -> FileArchiveResult<FileArchive> {
    let assets =
        match FileArchive::from_any_path(asset_paths) {
            Ok(result) => Ok(result),
            Err(err) => {
                log::info!("External asset loading failed ({:?})", err);
                open_embedded()
            }
        };
    log::info!("Loaded assets");
    assets
}

#[cfg(test)]
mod tests {
    use sp_asset::archive::FileArchive;
    use std::{
        io::{Cursor, Read},
        path::Path,
    };

    #[test]
    fn read_embedded_file() {
        let bytes = include_bytes!("../assets.zip");
        let cursor = Cursor::new(bytes.to_vec());
        let mut assets = FileArchive::from_memory(cursor).unwrap();
        // Read all files in folder
        let files = assets.files_in(Path::new("textures")).unwrap();
        assert!(!files.is_empty());
        for path in files.iter() {
            //println!("{:?}", path);
            let mut file = assets.open(path).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            assert!(!buf.is_empty());
        }
    }
}
