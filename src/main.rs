mod downloader;

use downloader::path_handler;
use rand::Rng;
use std::{fs::ReadDir, thread, time::Duration};
use rusty_audio::prelude::*;
use dirs::home_dir;

fn main() {
    thread::sleep(Duration::from_secs(20*60)); //спим 20 минут чтобы она не сразу орала
    loop {
        let mut rngl = rand::thread_rng();
        thread::sleep(Duration::new(rngl.gen_range(15 * 60..35 * 60), 0));
        let mut filesarr = Vec::new();
        
        let path = home_dir().unwrap().join(".sounds");
        
        let iter = match path_handler(&path) {
            Some(value) => value,
            None => continue,
        };
        
        //пробегаем по всем файлам в папке и на рандом выбираем звук
        let files = ReadDir::into_iter(iter);

        for i in files {
            let mut path = path.to_str().unwrap().to_string();
            path.push_str(i.unwrap().file_name().into_string().unwrap().as_str());
            println!("{path}");
            filesarr.push(path);
        }

        println!("{:?}", filesarr);
        // let mut rngl = rand::thread_rng();
        let num = rngl.gen_range(0..filesarr.len());
        play(filesarr[num].clone())
    }
}

fn play(path: String) {
    let mut audio = Audio::new();
    audio.add("sound", path);
    audio.play("sound");
    audio.wait()
}