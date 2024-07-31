use std::{
    fs::{DirBuilder, File, ReadDir},
    io::{Read, Write},
    net::TcpStream,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

pub fn path_handler(path: &PathBuf) -> Option<ReadDir> {
    if !path.exists() {
        DirBuilder::new() // if folder doesnt exist create .sounds dir and download sounds into it
            .recursive(true)
            .create(path)
            .unwrap();
        download_files(path)
    }
    let iter = match path.read_dir() {
        Ok(iter) => iter,
        Err(_) => {
            println!("cannot read directory");
            return None;
        }
    };
    Some(iter)
}

fn download_files(path: &Path) {
    //downloads json file with list of urls
    let resp = get("");
    let urls = parse_index(resp);

    for i in urls {
        //iterates over array or urls, download file from every url and write it.
        let resp = get(i.as_str());
        let parts: Vec<&str> = i.split('/').collect();
        let name = parts.last().unwrap().to_owned();
        drop(parts);

        write_file(path.to_path_buf(), name, resp.as_bytes())
    }
}

pub fn get(url: &str) -> String {
    // GET request using std::net
    let url_parts: Vec<&str> = url.split('/').collect();
    let host = url_parts[2];
    let path = "/".to_string() + &url_parts[3..].join("/");

    let mut stream = TcpStream::connect(host).unwrap();

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    let _ = stream.write_all(request.as_bytes());

    let mut response = String::new();
    let _ = stream.read_to_string(&mut response);

    response
}

pub fn parse_index(index: String) -> Vec<String> {
    //parses json file and returns an array of urls
    let parsed: JsonFromWeb = serde_json::from_str(index.as_str()).unwrap();
    parsed.urls
}

/// creates a file with name filename in path and write contents into it
fn write_file(path: PathBuf, filename: &str, contents: &[u8]) {
    let mut file = File::create(path.join(filename)).unwrap();
    let _ = file.write_all(contents);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json() {
        let index = String::from("{\"urls\": [\"http://example.com/\"]}");
        let parsed = parse_index(index);
        println!("{:?}", parsed);
        assert_eq!("http://example.com/", parsed[0])
    }
}

#[derive(Deserialize, Serialize)]
struct JsonFromWeb {
    // this json should contain urls list
    urls: Vec<String>,
}