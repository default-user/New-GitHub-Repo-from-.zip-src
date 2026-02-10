use crate::error::Zip2RepoError;
use bytes::Bytes;
use std::{
    fs,
    io::Cursor,
    path::PathBuf,
};
use tempfile::TempDir;
use url::Url;
use zip::ZipArchive;

pub async fn load_zip_bytes(zip_arg: &str) -> Result<Bytes, Zip2RepoError> {
    if zip_arg.starts_with("https://") {
        let _u = Url::parse(zip_arg)
            .map_err(|e| Zip2RepoError::Network(format!("bad url: {e}")))?;
        let resp = reqwest::Client::new()
            .get(zip_arg)
            .send()
            .await
            .map_err(|e| Zip2RepoError::Network(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(Zip2RepoError::Network(format!(
                "http status {}",
                resp.status()
            )));
        }
        resp.bytes()
            .await
            .map_err(|e| Zip2RepoError::Network(e.to_string()))
    } else {
        let b = fs::read(zip_arg).map_err(|e| Zip2RepoError::Io(e.to_string()))?;
        Ok(Bytes::from(b))
    }
}

pub fn open_zip(bytes: Bytes) -> Result<ZipArchive<Cursor<Bytes>>, Zip2RepoError> {
    let cur = Cursor::new(bytes);
    ZipArchive::new(cur).map_err(|e| Zip2RepoError::ZipValidation(e.to_string()))
}

pub fn extract_to_temp(
    mut zip: ZipArchive<Cursor<Bytes>>,
) -> Result<(TempDir, PathBuf), Zip2RepoError> {
    let td = TempDir::new().map_err(|e| Zip2RepoError::Io(e.to_string()))?;
    let root = td.path().join("src");
    fs::create_dir_all(&root).map_err(|e| Zip2RepoError::Io(e.to_string()))?;

    for i in 0..zip.len() {
        let mut f = zip
            .by_index(i)
            .map_err(|e| Zip2RepoError::ZipValidation(e.to_string()))?;
        let name = f.name().to_string();

        // directories
        if name.ends_with('/') {
            fs::create_dir_all(root.join(&name))
                .map_err(|e| Zip2RepoError::Io(e.to_string()))?;
            continue;
        }

        // default policy: ignore symlinks (best-effort; zip crate exposes unix_mode)
        if let Some(mode) = f.unix_mode() {
            let file_type = mode & 0o170000;
            let is_symlink = file_type == 0o120000;
            if is_symlink {
                // quarantine: skip
                continue;
            }
        }

        let out_path = root.join(&name);
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).map_err(|e| Zip2RepoError::Io(e.to_string()))?;
        }
        let mut out =
            fs::File::create(&out_path).map_err(|e| Zip2RepoError::Io(e.to_string()))?;
        std::io::copy(&mut f, &mut out).map_err(|e| Zip2RepoError::Io(e.to_string()))?;
    }

    Ok((td, root))
}
