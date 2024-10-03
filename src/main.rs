#![forbid(unsafe_code)]
#![windows_subsystem = "windows"]

mod downloader;

use downloader::path_handler;
use rand::Rng;
use std::{env, fs::{self, ReadDir}, path::PathBuf, thread, time::Duration};
use awedio::Sound;

fn main() {
    let path = env::temp_dir().join(".sounds/"); // the path of sounds dir.
    let _ = ctrlc::set_handler(|| {stop_and_clear(&env::temp_dir().join(".sounds"));});
    let url = env!("URL", "no url provided");
    let config = downloader::Config::from(&format!("{url}/config.json"));
    let url = &format!("{url}/index.json");
    
    thread::sleep(Duration::from_secs(config.cooldown * 60)); //sleeps for 20 mins
    
    loop {
        let iter = path_handler(&path, url);
        
        //iterating over all files in directory and picking a random sound
        let files = ReadDir::into_iter(iter);
        
        let mut filesarr = Vec::new();
        for i in files {
            let mut path = path.to_str().unwrap().to_string();
            path.push_str(i.unwrap().file_name().into_string().unwrap().as_str());
            filesarr.push(path);
        }
        
        let mut rngl = rand::thread_rng();
        thread::sleep(Duration::new(rngl.gen_range((config.cooldown * 60)..(config.cooldown * 60)), 0)); //sleeps randomly from 15 to 35 mins

        println!("{:?}", filesarr);
        let num = rngl.gen_range(0..filesarr.len()); //random index
        play_audio(&filesarr[num], config.volume)
    }
}

/// Plays sound from path
fn play_audio(path: &String, volume: f64) {
    let (mut manager, _) = awedio::start().unwrap();
        let (audio, _) = awedio::sounds::open_file(path).unwrap()
            .with_adjustable_volume_of(volume as f32)
            .pausable()
            .controllable();
        
    manager.play(Box::new(audio));
}


/// Clears all files and exits
pub fn stop_and_clear(path: &PathBuf) {    
    fs::remove_dir_all(path).unwrap();
    std::process::exit(0);
}