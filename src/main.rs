#![windows_subsystem = "windows"] // скрывает окно консоли только на винде, todo на линукс надо бы
use rand::Rng;
use std::{fs::ReadDir, path::Path, thread, time::Duration};
use rusty_audio::prelude::*;

fn main() {
    thread::sleep(Duration::from_secs(20*60)); //спим 20 минут чтобы она не сразу орала
    loop {
        let mut rngl = rand::thread_rng();
        thread::sleep(Duration::new(rngl.gen_range(15 * 60..35 * 60), 0));
        let mut filesarr = Vec::new();
        let path = Path::new("C:/music/mp3/"); // массив файлов собирается из этого пути 
        println!("{}", path.to_str().unwrap());
        if !path.exists() {
            println!("папки нету");
            continue;
        }
        let iter = match path.read_dir() {
            Ok(iter) => iter,
            Err(_) => {
                println!("нельзя получить данные о папке");
                continue;
            }
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
    audio.add("sound", path)
}