use std::path::*;

use crate::path::*;

pub fn create_file(file_name: &str) {
    let root_path = detect_or_create_mc_dir();
    let path = Path::new(root_path).join(file_name);
}

pub fn read_file() {
    todo!()
}

pub fn write_file() {
    todo!()
}

pub fn delete_file() {
    todo!()
}
