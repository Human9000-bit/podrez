use std::{
    fs::{self, File, ReadDir},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use smolhttp::get;

use serde::{Deserialize, Serialize};

pub fn path_handler(path: &PathBuf) -> Option<ReadDir> {
    if !path.exists() {
        fs::create_dir(path).expect("failed to crate dir");
        download_files(path)
    }
    let iter = match path.read_dir() {
        Ok(iter) => iter,
        Err(_) => {
            return None;
        }
    };
    Some(iter)
}

fn download_files(path: &Path) {
    let mut url = String::new();
    File::open("config").unwrap().read_to_string(&mut url).unwrap();
    //downloads json file with list of urls
    let resp = get(url.as_str()).unwrap().text();
    let urls = parse_index(resp);

    for i in urls {
        //iterates over array or urls, download file from every url and write it.
        let resp = get(i.as_str());
        let parts: Vec<&str> = i.split('/').collect();
        let name = parts.last().unwrap().to_owned();
        drop(parts);

        write_file(path.to_path_buf(), name, resp.unwrap().content().as_slice())
    }
}

pub fn parse_index(index: String) -> Vec<String> {
    //parses json file and returns an array of urls
    let result = serde_json::from_str(index.as_str());
    match result {
        Ok(j) => {let parsed: JsonFromWeb = j;
            parsed.urls}
        Err(..) => vec![],
    }
}

/// creates a file with name filename in path and write contents into it
fn write_file(path: PathBuf, filename: &str, contents: &[u8]) {
    let mut file = File::create(path.join(filename)).unwrap();
    let _ = file.write_all(contents);
}

#[cfg(test)]
mod tests {
    use std::{env, io::read_to_string};

    use super::*;

    #[test]
    fn test_json() {
        let index = String::from("{\"urls\": [\"http://example.com/\"]}");
        let parsed = parse_index(index);
        println!("{:?}", parsed);
        assert_eq!("http://example.com/", parsed[0])
    }

    #[test]
    fn test_parse_index_empty() {
        let index = String::new();
        let parsed = parse_index(index);
        assert!(parsed.is_empty())
    }

    #[test]
    fn test_write_file() {
        write_file(env::current_dir().unwrap(), "file.txt", b"Hello, World!".as_slice());
        let contents = read_to_string(File::open("file.txt").unwrap()).unwrap();
        assert_eq!(contents, "Hello, World!")
    }
}

#[derive(Deserialize, Serialize)]
struct JsonFromWeb {
    // this json should contain urls list
    urls: Vec<String>,
}