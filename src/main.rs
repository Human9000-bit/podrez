#![windows_subsystem = "windows"]

mod downloader;

use downloader::path_handler;
use rand::Rng;
use std::{env, fs::{self, ReadDir}, path::PathBuf, thread, time::Duration};
use windows::Win32::Foundation::S_OK;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::core::GUID;
use std::io::{self, BufReader};
use std::path::Path;


fn main() {
    unsafe{
        let path = env::temp_dir().join(".sounds/"); // the path of sounds dir
        let _ = ctrlc::set_handler(|| {stop_and_clear(&env::temp_dir().join(".sounds"));});
        let _ = fs::remove_dir_all(&path);
        let url = env!("URL", "no url provided");
        let config = downloader::Config::from(&format!("{url}/config.json"));
        let url = &format!("{url}/index.json");
        
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
            thread::sleep(Duration::new(rngl.gen_range((config.min_cooldown)..(config.max_cooldown)), 0));

            println!("{:?}", filesarr);
            let num = rngl.gen_range(0..filesarr.len()); //random index

            let volume = match get_volume(){
                Ok(a)=>a,
                Err(_)=>continue,
            };

            thread::spawn(move ||match play_audio(Path::new(&filesarr[num]).to_path_buf(), config.volume as f32){
                Ok(_)=>(),
                Err(_)=>(),
            }).join().unwrap();
            
            match set_volume(volume){
                Ok(_)=>(),
                Err(_)=>continue,
            }

        }
    }
}

unsafe fn get_volume()->Result<f32,()> {
    CoUninitialize();
    match CoInitialize(None){
        S_OK=>(),
        _=>return Err(()),
    };
    let devicenum:IMMDeviceEnumerator = match CoCreateInstance(&MMDeviceEnumerator, None,CLSCTX_INPROC_SERVER){
        Ok(a) => {a},
        Err(_) => {return Err(());},
    };
    let defaultdevice = match devicenum.GetDefaultAudioEndpoint(eRender,eConsole){
        Ok(a)=>a,
        Err(_)=>return Err(())
    };
    let endpointval:IAudioEndpointVolume = match defaultdevice.Activate(CLSCTX_INPROC_SERVER,None){
        Ok(a) => a,
        Err(_) => return Err(()),
    };
    match endpointval.GetMasterVolumeLevelScalar(){
        Ok(a)=>Ok(a),
        Err(_)=>Err(())
    }
}

unsafe fn set_volume(vol:f32)->Result<(),()>{
    CoUninitialize();
    match CoInitialize(None){
        S_OK=>(),
        _=>return Err(()),
    };

    let devicenum:IMMDeviceEnumerator = match CoCreateInstance(&MMDeviceEnumerator, None,CLSCTX_INPROC_SERVER){
        Ok(a) => {a},
        Err(_) => {return Err(());},
    };

    let defaultdevice = match devicenum.GetDefaultAudioEndpoint(eRender,eConsole){
        Ok(a)=>a,
        Err(_)=>return Err(())
    };

    let endpointval:IAudioEndpointVolume = match defaultdevice.Activate(CLSCTX_INPROC_SERVER,None){
        Ok(a) => a,
        Err(_) => return Err(()),
    };

    endpointval.SetMute(false, 0 as *const GUID);
    match endpointval.SetMasterVolumeLevelScalar(vol, 0 as *const GUID){
        Ok(_)=>Ok(()),
        Err(_)=>Err(())
    }
}

// Plays sound from path
unsafe fn play_audio(path: PathBuf,vol:f32)->Result<(), ()>{
    match set_volume(vol){
        Ok(_)=>(),
        Err(_)=>return Err(()),
    }

    let (_stream, handle) = match rodio::OutputStream::try_default(){
        Ok((result,han)) => (result,han),
        Err(x) => return Err(()),
    };

    let sink = match rodio::Sink::try_new(&handle){
        Ok(result) => result,
        Err(x) => return Err(()),
    };

    let file =  match std::fs::File::open(path){
        Ok(result) => result,
        Err(x) => return Err(()),
    };

    sink.append(match rodio::Decoder::new(BufReader::new(file)){
        Ok(result) => result,
        Err(x) => return Err(()),
    });

    sink.sleep_until_end();
    return Ok(());
}

/// Clears all files and exits
pub fn stop_and_clear(path: &PathBuf) {    
    fs::remove_dir_all(path).unwrap();
    std::process::exit(0);
}