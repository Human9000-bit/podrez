#![forbid(unsafe_code)]
#![windows_subsystem = "windows"]

mod downloader;

use awedio::Sound;
use downloader::path_handler;
use rand::Rng;
use std::{env, fs, path::PathBuf, thread, time::Duration};

#[smol_potat::main]
async fn main() {
    let path = env::temp_dir().join(".sounds/"); // the path of sounds dir.
    let _ = ctrlc::set_handler(|| {
        stop_and_clear(&env::temp_dir().join(".sounds"));
    });

    let _ = fs::remove_dir_all(&path);
    let url = env!("URL", "no url provided");
    let iter = path_handler(&path, format!("{}/index.json", url));
    let config = downloader::Config::from(&format!("{url}/config.json"));
    let files = iter.await;

    let mut filesarr: Vec<PathBuf> = Vec::new();

    files.for_each(|i| {
        let mut path = path.clone();
        path.push(i.unwrap().file_name().into_string().unwrap().as_str());
        filesarr.push(path);
    });

    loop {
        let mut rngl = rand::thread_rng();
        thread::sleep(Duration::from_secs(
            rngl.gen_range((config.min_cooldown)..(config.max_cooldown * 60)),
        ));

        println!("{:?}", filesarr);
        let num = rngl.gen_range(0..filesarr.len()); //random index
        let _ = play_audio(filesarr[num].clone(), config.volume);
    }
}

/// Plays sound from path
fn play_audio(path: PathBuf, volume: f64) -> Result<(), anyhow::Error> {
    let (mut manager, _backend) = awedio::start()?;
    let (audio, mut controller) = awedio::sounds::open_file(path)?
        .with_adjustable_volume_of(volume as f32)
        .pausable()
        .controllable();

    controller.set_volume(volume as f32);
    manager.play(Box::new(audio));
    thread::sleep(Duration::from_secs(10));
    Ok(())
}

/// Clears all files and exits
pub fn stop_and_clear(path: &PathBuf) {
    fs::remove_dir_all(path).unwrap();
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{downloader, play_audio};

    ///Tests that audio plays correctly
    #[smol_potat::test]
    async fn test_play_audio() {
        let mut path = env::current_dir().unwrap();
        downloader::download_and_write("https://download.samplelib.com/mp3/sample-3s.mp3", &path)
            .await;
        path.push("sample-3s.mp3");
        play_audio(path, 0.1).unwrap();
    }
}
