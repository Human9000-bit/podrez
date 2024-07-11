#![windows_subsystem = "windows"] //хуйня которая скрывает окно консоли
                                  //для того чтобы скомпилировать этот код нормально нужно установить cargo и создать проект(cargo new в консоли) и в файле Cargo.toml в поле [dependencies] вставить:
                                  //rodio = "0.19.0"
                                  //rand = "0.8.5"
use rand::Rng;
use rodio; //библиотека которая звуки включает
use std::fs::ReadDir;
use std::io::BufReader;
use std::{path::Path, thread, time::Duration};

// чтобы скомпилировать код cargo build или cargo build --release, чтобы она меньше места в оперативке хавала и была оптимизированее
fn main() {
    thread::sleep(Duration::new(20 * 60, 0)); //спим, чтобы она не сразу орала
    loop {
        let mut rngl = rand::thread_rng();
        let mut filesarr = Vec::new(); //массив для хранения файлов в C:/music/mp3/
        thread::sleep(Duration::new(rngl.gen_range(15 * 60..35 * 60), 0));
        let path = Path::new("C:/music/mp3/");
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
            println!("{}", path);
            filesarr.push(path);
        }
        println!("{:?}", filesarr);
        // let mut rngl = rand::thread_rng();
        let num = rngl.gen_range(0..filesarr.len());
        match play_mp3(filesarr[num].clone()) {
            Ok(_) => println!("звук проигран"),
            Err(e) => println!("звук не проигран: {:?}", e),
        }
    }
}
//спиздил из другово своего проекта функцию, которая играет mp3 файлы
fn play_mp3(path: String) -> Result<Vec<u8>, Vec<u8>> {
    let (_stream, handle) = match rodio::OutputStream::try_default() {
        Ok((result, han)) => (result, han),
        Err(x) => return Err(x.to_string().as_bytes().to_vec()),
    };
    let sink = match rodio::Sink::try_new(&handle) {
        Ok(result) => result,
        Err(x) => return Err(x.to_string().as_bytes().to_vec()),
    };

    let file = match std::fs::File::open(path) {
        Ok(result) => result,
        Err(x) => return Err(x.to_string().as_bytes().to_vec()),
    };
    sink.append(match rodio::Decoder::new(BufReader::new(file)) {
        Ok(result) => result,
        Err(x) => return Err(x.to_string().as_bytes().to_vec()),
    });

    sink.sleep_until_end();
    Ok("play_mp3 succes".as_bytes().to_vec())
}
