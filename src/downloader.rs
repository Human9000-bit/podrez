use std::{
    fs::{self, File, ReadDir},
    io::Write,
    path::{Path, PathBuf},
};
use smolhttp::get;
use serde::{Deserialize, Serialize};

/// Hadles provided path. Returns ReadDir iter if success.
///
/// # Panics
///
/// Panics if there is no files in dir
pub fn path_handler(path: &PathBuf, url: &str) -> ReadDir {
    if !path.exists() {
        fs::create_dir(path).expect("failed to crate dir");
        download_files(path, url)
    }
    
    match path.read_dir() {
        Ok(iter) => iter,
        Err(e) => panic!("{:?}", e)
    }
}

/// Parses json from response of url, then download and write files from all urls in json
fn download_files(path: &Path, url: &str) {
    let resp = get(url).unwrap().text();
    let urls = parse_index(resp);

    //iterates over array or urls, download file from every url and write it.
    for i in urls {
        let resp = get(i.as_str()).unwrap();
        println!("{:?}", &resp);
        let parts: Vec<&str> = i.split('/').collect();
        let name = parts.last().unwrap().to_owned();

        write_file(path.to_path_buf(), name, resp.content().as_slice())
    }
}

/// Parses json file and returns an array of urls
pub fn parse_index(index: String) -> Vec<String> {
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
    
    // #[test]
    // fn test_get() {
    //     let resp = get("https://mipoh.furryporno.ru/bass.mp3").unwrap().content();
    //     assert_eq!(resp.as_slice(), [10])
    // }
}

/// Structure of the json. Must contain only array of url strings, or panics otherwise.
#[derive(Deserialize, Serialize)]
struct JsonFromWeb {
    urls: Vec<String>,
}