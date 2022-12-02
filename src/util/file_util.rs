use std::path::{Path, PathBuf};
use std::{fs, fs::File};
use std::io::{Read, Write};

#[allow(dead_code)]
pub fn read_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

#[allow(dead_code)]
pub fn write_file(path: &Path, text: &str) {
    let mut file = File::create(path).unwrap_or_else(|e| {
        panic!("Could not create file: {:?}", e);
    });
    file.write_all(text.as_bytes()).unwrap_or_else(|e| {
        panic!("Write file: {:?}", e);
    });
}

#[allow(dead_code)]
pub fn create_dir(path: &Path) {
    fs::create_dir_all(path).unwrap_or_else(|e| {
        panic!("Could not create file directory: {}, {:?}", &path.display(), e)
    });
}

#[allow(dead_code)]
pub fn create_file(path: &Path) {
    if !path.parent().unwrap().exists() {
        let _ = fs::create_dir_all(path.parent().unwrap());
    }
    if !path.exists() {
        let _ = File::create(path);
    }
}

#[allow(dead_code)]
pub fn list_dir(path: &Path) -> Vec<PathBuf> {
    let mut list = Vec::default();
    for child in fs::read_dir(path).unwrap() {
        list.push(child.unwrap().path());
    }
    list
}