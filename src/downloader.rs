use std::{fs::{DirBuilder, ReadDir}, path::PathBuf};

pub fn path_handler(path: &PathBuf) -> Option<ReadDir> {
    if !path.exists() {
        DirBuilder::new()
            .create(".sounds").unwrap();
        download_files(path)
    }
    let iter = match path.read_dir() {
        Ok(iter) => iter,
        Err(_) => {
            println!("нельзя получить данные о папке");
            return None;
        }
    };
    Some(iter)
}

pub fn download_files(path: &PathBuf) {
    
}