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
    let _ = ctrlc::set_handler(|| {
        stop_and_clear(&env::temp_dir().join(".sounds"));
    });

    println!("{:?}", path);
    
    let iter = path_handler(&path, format!("{}/index.json", URL));
    let config = downloader::Config::from_url(format!("{URL}/config.json"));

    let files = iter.await?;
    let filesarr: Vec<PathBuf> = files.map(|i| i.unwrap().path()).collect();

    let config = config.await?;
    
    main_loop(config, filesarr).await;
    
    stop_and_clear(&path);
    
    Ok(())
}

async fn main_loop(config: downloader::Config, filesarr: Vec<PathBuf>) {
    loop {
        let mut rngl = rand::thread_rng();
        thread::sleep(Duration::from_secs(
            match config.min_cooldown >= config.max_cooldown || config.max_cooldown == 0 {
                true => config.min_cooldown,
                false => rngl.gen_range(config.min_cooldown..config.max_cooldown),
            },
        ));

        let num = rngl.gen_range(0..filesarr.len()); //random index
        println!("playing: {:?}", filesarr[num]);
        let _ = play_audio(filesarr[num].clone(), config.volume).await;
    }
}

/// Clears all files and exits
pub fn stop_and_clear(path: &PathBuf) {
    let _ = fs::remove_dir_all(path);
    std::process::exit(0);
}
