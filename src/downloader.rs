use futures::future::join_all;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    env,
    fs::{self, read_dir, write, ReadDir},
    path::{Path, PathBuf},
};
use ureq::get;

use crate::stop_and_clear;

/// Hadles provided path. Returns ReadDir iter if success.
///
/// Panics if there is no files in dir
pub async fn path_handler(path: &PathBuf, url: String) -> Result<ReadDir, anyhow::Error> {
    match !path.exists() || path.read_dir()?.count() == 0 {
        true => {
            println!("no files in dir, downloading...");
            let _ = fs::create_dir(path);
            download_files(path, url.as_str()).await?;
        }
        false => {
            println!("found mp3s in dir")
        }
    }

    let read_dir = read_dir(path)?;
    println!("download complete");
    Ok(read_dir)
}

/// Parses json from response of url, then download and write files from all urls in json
async fn download_files(path: &Path, url: &str) -> Result<(), anyhow::Error> {
    let resp = get(url)
        .call()
        .inspect_err(|_| stop_and_clear(&env::temp_dir().join(".sounds")));

    let resp = resp?.into_string()?;
    let urls = parse_index(resp).unwrap();

    //iterates over array or urls, download file from every url and write it.
    let hadles = urls.iter().map(|i| download_and_write(i, path));
    join_all(hadles).await; //spawns all async handles and executes in one time
    Ok(())
}

///Downloads file from url and writes into the path
pub async fn download_and_write(url: &str, path: &Path) -> Result<(), anyhow::Error> {
    let mut resp = Vec::new();
    get(url).call()?.into_reader().read_to_end(&mut resp)?;
    let parts: Vec<&str> = url.split('/').collect();
    let name = parts.last().unwrap_or(&"file.mp3");
    println!("downloaded {name}");

    write_file(path.to_path_buf(), name, resp.as_slice())?;
    Ok(())
}

///Parses json file and returns an array of urls
fn parse_index(index: String) -> Option<Vec<String>> {
    let result = serde_json::from_str(index.as_str());
    match result {
        Ok(j) => {
            let parsed: JsonFromWeb = j;
            Some(parsed.urls)
        }
        Err(_) => None,
    }
}

/// creates a file with name filename in path and write contents into it
fn write_file(path: PathBuf, filename: &str, contents: &[u8]) -> Result<(), anyhow::Error> {
    write(path.join(filename), contents)?;
    Ok(())
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
    pub volume: f64,
}

impl Config {
    /// Parses config from downloaded url
    pub async fn from(url: String) -> Result<Self, anyhow::Error> {
        let path = env::temp_dir().join(".sounds");
        let resp = get(url.as_str())
            .call()
            .inspect_err(|_e| stop_and_clear(&path))
            .unwrap()
            .into_string()
            .inspect_err(|_e| stop_and_clear(&path))
            .unwrap();
        let json: Value = serde_json::from_str(&resp).unwrap();
        Ok(Self {
            min_cooldown: json["min_cooldown"].as_i64().unwrap_or(60) as u64,
            max_cooldown: json["max_cooldown"].as_i64().unwrap_or(600) as u64,
            volume: json["volume"].as_f64().unwrap_or(1.0),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{env, io::read_to_string};

    use super::*;

    /// Test that parses example.com from index
    #[smol_potat::test]
    async fn test_json() {
        let index = String::from("{\"urls\": [\"http://example.com/\"]}");
        let parsed = parse_index(index).unwrap();
        println!("{:?}", parsed);
        assert_eq!("http://example.com/", parsed[0])
    }

    /// Test that parses empty string
    #[smol_potat::test]
    async fn test_parse_index_empty() {
        let index = String::new();
        let parsed = parse_index(index);
        assert_eq!(parsed, None)
    }

    /// Test that writes file and reads it
    ///
    /// Panics if failed to read or incorrect string read
    #[smol_potat::test]
    async fn test_write_file() {
        write_file(
            env::current_dir().unwrap(),
            "file.txt",
            b"Hello, World!".as_slice(),
        )
        .unwrap();
        let contents = read_to_string(std::fs::File::open("file.txt").unwrap()).unwrap();
        assert_eq!(contents, "Hello, World!")
    }
}
