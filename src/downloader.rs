use std::{
    env, fs::{self, write, ReadDir}, path::{Path, PathBuf}};
use futures::future::join_all;
use serde_json::Value;
use ureq::get;
use serde::{Deserialize, Serialize};

use crate::stop_and_clear;

/// Hadles provided path. Returns ReadDir iter if success.
/// 
/// Panics if there is no files in dir
pub async fn path_handler(path: &PathBuf, url: &str) -> ReadDir {
    if !path.exists() || path.read_dir().unwrap().count() == 0 {
        println!("no files in dir, downloading...");
        let _ = fs::create_dir(path);
        download_files(path, url).await
    } else {println!("found mp3s in dir")}
    
    match path.read_dir() {
        Ok(iter) => iter,
        Err(e) => panic!("{:?}", e)
    }
}

/// Parses json from response of url, then download and write files from all urls in json
async fn download_files(path: &Path, url: &str) {
    let resp = get(url).call().inspect_err(|_e| stop_and_clear(&env::temp_dir().join(".sounds")));
    
    let resp = resp.unwrap().into_string().unwrap();
    let urls = parse_index(resp);

    //iterates over array or urls, download file from every url and write it.
    let mut hadles = vec![];
    for i in urls.as_slice() {
        hadles.push(download_and_write(i, path))
    }
    join_all(hadles).await; //spawns all async handles and executes in one time 
}

///Downloads file from url and writes into the path
async fn download_and_write(url: &str, path: &Path) {
    let mut resp = Vec::new();
    get(url).call().expect("failed to download mp3").into_reader().read_to_end(&mut resp).expect("failed to convert");
    let parts: Vec<&str> = url.split('/').collect();
    let name = parts.last().unwrap();

    write_file(path.to_path_buf(), name, resp.as_slice())
}

///Parses json file and returns an array of urls
pub fn parse_index(index: String) -> Vec<String> {
    let result = serde_json::from_str(index.as_str());
    match result {
        Ok(j) => {let parsed: JsonFromWeb = j;
            parsed.urls}
        Err(_) => vec![],
    }
}

/// creates a file with name filename in path and write contents into it
fn write_file(path: PathBuf, filename: &str, contents: &[u8]) {
    let _ = write(path.join(filename), contents);
}

#[cfg(test)]
mod tests {
    use std::{env, io::read_to_string};

    use super::*;

    /// Test that parses example.com from index
    #[test]
    fn test_json() {
        let index = String::from("{\"urls\": [\"http://example.com/\"]}");
        let parsed = parse_index(index);
        println!("{:?}", parsed);
        assert_eq!("http://example.com/", parsed[0])
    }

    /// Test that parses empty string
    #[test]
    fn test_parse_index_empty() {
        let index = String::new();
        let parsed = parse_index(index);
        assert!(parsed.is_empty())
    }

    /// Test that writes file and reads it
    /// 
    /// Panics if failed to read or incorrect string read
    #[test]
    fn test_write_file() {
        write_file(env::current_dir().unwrap(), "file.txt", b"Hello, World!".as_slice());
        let contents = read_to_string(std::fs::File::open("file.txt").unwrap()).unwrap();
        assert_eq!(contents, "Hello, World!")
    }
}

/// Structure of the json. Must contain only array of url strings, or panics otherwise.
#[derive(Deserialize, Serialize)]
struct JsonFromWeb {
    urls: Vec<String>,
}

/// Config structure
pub struct Config {
    pub min_cooldown: u64,
    pub max_cooldown: u64,
    pub volume: f64
}

impl Config {
    /// Parses config from downloaded url
    pub fn from(url: &str) -> Self {
        let path = env::temp_dir().join(".sounds");
        let resp = get(url).call().inspect_err(|_e| stop_and_clear(&path))
            .unwrap().into_string().inspect_err(|_e| stop_and_clear(&path)).unwrap();
        let json: Value = serde_json::from_str(&resp).unwrap();
        Self {
            min_cooldown: json["min_cooldown"].as_i64().unwrap_or(6) as u64,
            max_cooldown: json["max_cooldown"].as_i64().unwrap_or(20) as u64,
            volume: json["volume"].as_f64().unwrap_or(1.0)
        }
    }
}