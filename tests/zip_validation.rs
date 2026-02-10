use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

/// Helper: create an in-memory zip with the given entries.
fn make_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    {
        let mut zw = ZipWriter::new(&mut buf);
        let options = SimpleFileOptions::default();
        for (name, data) in entries {
            zw.start_file(*name, options).unwrap();
            zw.write_all(data).unwrap();
        }
        zw.finish().unwrap();
    }
    buf.into_inner()
}

#[test]
fn valid_zip_passes() {
    let data = make_zip(&[("hello.txt", b"hello world")]);
    let cursor = Cursor::new(bytes::Bytes::from(data));
    let archive = zip::ZipArchive::new(cursor).unwrap();
    let result = zip2repo::zip::validate::validate_zip(archive);
    assert!(result.is_ok());
}

#[test]
fn absolute_path_rejected() {
    let data = make_zip(&[("/etc/passwd", b"nope")]);
    let cursor = Cursor::new(bytes::Bytes::from(data));
    let archive = zip::ZipArchive::new(cursor).unwrap();
    let result = zip2repo::zip::validate::validate_zip(archive);
    assert!(result.is_err());
    let msg = format!("{}", result.unwrap_err());
    assert!(msg.contains("absolute path"));
}

#[test]
fn path_traversal_rejected() {
    let data = make_zip(&[("foo/../../../etc/passwd", b"nope")]);
    let cursor = Cursor::new(bytes::Bytes::from(data));
    let archive = zip::ZipArchive::new(cursor).unwrap();
    let result = zip2repo::zip::validate::validate_zip(archive);
    assert!(result.is_err());
    let msg = format!("{}", result.unwrap_err());
    assert!(msg.contains("path traversal"));
}

#[test]
fn multiple_valid_entries() {
    let data = make_zip(&[
        ("src/main.rs", b"fn main() {}"),
        ("Cargo.toml", b"[package]"),
        ("README.md", b"# Hello"),
    ]);
    let cursor = Cursor::new(bytes::Bytes::from(data));
    let archive = zip::ZipArchive::new(cursor).unwrap();
    let result = zip2repo::zip::validate::validate_zip(archive);
    assert!(result.is_ok());
}
