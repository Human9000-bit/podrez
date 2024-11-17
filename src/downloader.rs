use anyhow::Ok;
use futures::future::join_all;
use serde::Deserialize;
use smol::fs::{self, write};
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};
use ureq::get;

/// Hadles provided path. Returns ReadDir iter if success.
///
/// Panics if there is no files in dir
pub async fn path_handler(path: &PathBuf, url: String) -> Result<std::fs::ReadDir, anyhow::Error> {
    match !path.exists() || path.read_dir()?.count() == 0 {
        true => {
            println!("no files in dir, downloading...");
            let _ = fs::create_dir(path).await;
            download_files(path, url.as_str()).await?;
            println!("download complete");
        }
        false => {
            println!("found mp3s in dir")
        }
    }

    let read_dir = read_dir(path)?;
    Ok(read_dir)
}

/// Parses json from response of url, then download and write files from all urls in json
async fn download_files(path: &Path, url: &str) -> Result<(), anyhow::Error> {
    let resp = get(url).call();

    let resp = match resp {
        Result::Ok(r) => r,
        Err(_) => get(url).call()?,
    };

    let resp = resp.into_string()?;
    let urls = parse_index(resp).await.unwrap();

    //iterates over array or urls, download file from every url and write it.
    let hadles = urls.urls.iter().map(|i| download_and_write(i, path));
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

    write_file(path.to_path_buf(), name, resp.as_slice()).await?;
    Ok(())
}

///Parses json file and returns an array of urls
async fn parse_index(index: String) -> Option<JsonFromWeb> {
    let result: Result<JsonFromWeb, serde_json::Error> = serde_json::from_str(index.as_str());
    result.ok()
}

/// creates a file with name filename in path and write contents into it
async fn write_file(path: PathBuf, filename: &str, contents: &[u8]) -> Result<(), anyhow::Error> {
    write(path.join(filename), contents).await?;
    Ok(())
}

/// Structure of the json. Must contain only array of url strings, or panics otherwise.
#[derive(Deserialize, Clone)]
struct JsonFromWeb {
    urls: Vec<String>,
}

/// Config structure
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Config {
    pub min_cooldown: u64,
    pub max_cooldown: u64,
    pub volume: f64,
}

impl Config {
    /// Parses config from downloaded url
    pub async fn from_url(url: String) -> Result<Self, anyhow::Error> {
        let resp = get(url.as_str()).call()?.into_string()?;
        let json: Self = serde_json::from_str(&resp)?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use std::{env, io::read_to_string};

    use super::*;

    /// Test that parses example.com from index
    #[smol_potat::test]
    async fn test_json() {
        let index = String::from(r#"{
            "urls": ["http://example.com/"]
            }"#);
        let parsed = parse_index(index).await.unwrap().urls;
        println!("{:?}", parsed);
        assert_eq!("http://example.com/", parsed[0])
    }

    /// Test that parses empty string
    #[smol_potat::test]
    async fn test_parse_index_empty() {
        let index = String::new();
        let parsed = parse_index(index).await;
        assert!(parsed.is_none())
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
        .await
        .unwrap();
        let contents = read_to_string(std::fs::File::open("file.txt").unwrap()).unwrap();
        assert_eq!(contents, "Hello, World!")
    }
}
