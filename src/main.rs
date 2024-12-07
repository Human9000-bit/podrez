#![windows_subsystem = "windows"]

mod downloader;
mod sound;

use downloader::path_handler;
use rand::Rng;
use sound::play_audio;
use std::{env, fs, path::PathBuf, thread, time::Duration};

const URL: &str = env!("URL", "no url provided");

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    let path = env::temp_dir().join(".sounds"); // the path of sounds dir.
    
    // set ctrlc signal handler
    let _ = ctrlc::set_handler(|| {
        stop_and_clear(&env::temp_dir().join(".sounds"));
    });

    println!("{:?}", path);
    
    // download index.json and config.json nad get ReadDir of audio dir
    let iter = path_handler(&path, format!("{}/index.json", URL));
    
    // initialize config from url
    let config = downloader::Config::from_url(format!("{URL}/config.json"));

    // get array of files from ReadDir
    let files = iter.await?;
    let filesarr: Vec<PathBuf> = files.map(|i| i.unwrap().path()).collect();

    let config = config.await?;
    
    main_loop(config, filesarr).await;
    
    // when the main loop ends, clear the dir and exit
    stop_and_clear(&path);
    
    Ok(())
}

/// The main loop of the program
/// 
/// Plays random audio from the array of files and sleeps for random time
async fn main_loop(config: downloader::Config, filesarr: Vec<PathBuf>) {
    loop {
        let mut rngl = rand::thread_rng();
        if config.max_cooldown == 0 {continue;}
        thread::sleep(Duration::from_secs(
            // if min cooldown is bigger than max cooldown or max cooldown is 0 then use min cooldown
            match config.min_cooldown >= config.max_cooldown{
                true => config.min_cooldown,
                false => rngl.gen_range(config.min_cooldown..config.max_cooldown),
            },
        ));

        // get random element from array
        let num = rngl.gen_range(0..filesarr.len());
        println!("playing: {:?}", filesarr[num]);
        
        let _ = play_audio(filesarr[num].clone(), config.volume).await;
    }
}

/// Clears all files and exits the program
pub fn stop_and_clear(path: &PathBuf) {
    let _ = fs::remove_dir_all(path);
    std::process::exit(0);
}
