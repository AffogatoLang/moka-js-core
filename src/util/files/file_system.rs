use std::path::{Path, PathBuf};
use std::env;
use std::io::prelude::*;
use std::io;
use std::fs;
use interop::osstr_to_string;

pub fn read_as_file(path: &Path) -> io::Result<(String, String)> {
    let mut file : fs::File = fs::File::open(path)?;
    let mut buffer = &mut String::new();
    file.read_to_string(buffer)?;

    Ok((osstr_to_string(path.file_stem().unwrap()).unwrap(), buffer.clone()))
}