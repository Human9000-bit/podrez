mod downloader;

use downloader::path_handler;
use rand::Rng;
use std::{fs::ReadDir, thread, time::Duration};
use rusty_audio::prelude::*;
use dirs::home_dir;

fn main() {
    thread::sleep(Duration::from_secs(20*60)); //sleeps for 20 mins
    loop {
        let mut rngl = rand::thread_rng();
        thread::sleep(Duration::new(rngl.gen_range(15 * 60..35 * 60), 0)); //sleeps randomly from 15 to 35 mins
        let mut filesarr = Vec::new();
        
        let path = home_dir().unwrap().join(".sounds"); // the path of sounds dir. used dir crate because of diiferenco of home dr in unix and windows
        
        let iter = match path_handler(&path) {
            Some(value) => value,
            None => continue,
        };
        
        //iterating over all files in directory and picking a random sound
        let files = ReadDir::into_iter(iter);

        for i in files {
            let mut path = path.to_str().unwrap().to_string();
            path.push_str(i.unwrap().file_name().into_string().unwrap().as_str());
            println!("{path}");
            filesarr.push(path);
        }

        println!("{:?}", filesarr);
        let num = rngl.gen_range(0..filesarr.len()); //random index
        play(filesarr[num].clone())
    }
}

fn play(path: String) { //plays the very sound
    let mut audio = Audio::new();
    audio.add("sound", path);
    audio.play("sound");
    audio.wait()
}