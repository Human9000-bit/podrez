use std::{fs::{DirBuilder, ReadDir}, io::{Read, Write}, net::TcpStream, path::{PathBuf, Path}};

pub fn path_handler(path: &PathBuf) -> Option<ReadDir> {
    if !path.exists() {
        DirBuilder::new() // if folder doesnt exist create .sounds dir and download sounds into it
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

pub fn download_files(path: &Path) { //downloads json file with list of urls
    let resp = get("");
    let urls = parse_index(resp);
    
    for i in urls { //iterates over array or urls, download file from every url and write it.
        let resp = get(i);
        let parts: Vec<&str> = i.split('/').collect();
        let name = parts.last();
        drop(parts);
        
        write_file
    }
}

pub fn get(url: &str) -> String { // GET request using std::net
    let url_parts: Vec<&str> = url.split('/').collect();
    let host = url_parts[2];
    let path = "/".to_string() + &url_parts[3..].join("/");
    
    let mut stream = TcpStream::connect(host).unwrap();
    
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    stream.write_all(request.as_bytes());
    
    let mut response = String::new();
    stream.read_to_string(&mut response);
    
    response
}

pub fn parse_index(index: String) -> Vec<&'static str> { //parses json file and returns an array of urls
    let str: &str = &index;
    let parsed = json::parse(str);
    
    parsed.unwrap();
}

fn write_file(path: PathBuf, filename: &str) {
    
}