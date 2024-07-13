use core::panicking::panic;
use std::{error::Error, fs::{DirBuilder, ReadDir}, io::{Read, Write}, net::TcpStream, path::{PathBuf, Path}};


pub fn path_handler(path: &PathBuf) -> Option<ReadDir> {
    if !path.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(path).unwrap();
        
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

pub fn download_files(path: &Path) {
    match get("https://mipoh.ru/podrez/index.json") {
        Err(e) => panic!("{:?}", e),
        
        Ok(index) => {let urls = parse_index(index);}
    };
}

pub fn get(url: &str) -> Result<String, Box<dyn Error>> {
    let url_parts: Vec<&str> = url.split("/").collect();
    let host = url_parts[2];
    let path = "/".to_string() + &url_parts[3..].join("/");
    
    let mut stream = TcpStream::connect(host)?;
    
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    stream.write_all(request.as_bytes())?;
    
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    
    Ok(response)
}

pub fn parse_index(index: String) -> Vec<&str> {
    
}