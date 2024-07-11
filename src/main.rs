#![windows_subsystem = "windows"] // скрывает окно консоли только на винде, todo на линукс надо бы
use rand::Rng;
use std::{fs::ReadDir, io::BufReader, path::Path, thread, time::Duration};

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
