use crate::error::Zip2RepoError;
use std::io::{Read, Seek};
use zip::ZipArchive;

const MAX_UNCOMPRESSED_BYTES: u64 = 512 * 1024 * 1024;
const MAX_FILE_COUNT: u64 = 50_000;

pub fn validate_zip<R: Read + Seek>(mut zip: ZipArchive<R>) -> Result<(), Zip2RepoError> {
    let mut total: u64 = 0;
    let mut count: u64 = 0;

    for i in 0..zip.len() {
        let f = zip
            .by_index(i)
            .map_err(|e| Zip2RepoError::ZipValidation(e.to_string()))?;
        let name = f.name().to_string();

        // zip-slip / traversal checks
        if name.starts_with('/') || name.starts_with('\\') {
            return Err(Zip2RepoError::ZipValidation(format!(
                "absolute path entry rejected: {name}"
            )));
        }
        if name.contains(':') && name.chars().nth(1) == Some(':') {
            return Err(Zip2RepoError::ZipValidation(format!(
                "drive-letter path rejected: {name}"
            )));
        }
        // normalize-ish: reject any .. segment
        if name.split('/').any(|seg| seg == "..")
            || name.split('\\').any(|seg| seg == "..")
        {
            return Err(Zip2RepoError::ZipValidation(format!(
                "path traversal rejected: {name}"
            )));
        }

        count += 1;
        if count > MAX_FILE_COUNT {
            return Err(Zip2RepoError::ZipValidation(
                "zip file count exceeds limit".into(),
            ));
        }

        let sz = f.size();
        total = total.saturating_add(sz);
        if total > MAX_UNCOMPRESSED_BYTES {
            return Err(Zip2RepoError::ZipValidation(
                "zip uncompressed bytes exceeds limit".into(),
            ));
        }
    }

    Ok(())
}
