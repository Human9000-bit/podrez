use awedio::Sound;
use std::thread;
use std::{path::PathBuf, time::Duration};

/// Plays sound from path
pub fn play_audio(path: PathBuf, volume: f64) -> Result<(), anyhow::Error> {
    let (mut manager, _backend) = awedio::start()?;

    let (audio, _controller) = awedio::sounds::open_file(path)?
        .with_adjustable_volume_of(volume as f32)
        .controllable();

    #[cfg(target_os = "windows")]
    {
        set_win_volume(volume)?
    }

    manager.play(Box::new(audio));
    thread::sleep(Duration::from_secs(10));
    Ok(())
}

/// Function that sets windows volume
#[cfg(target_os = "windows")]
unsafe fn set_win_volume(volume: f64) -> Result<(), anyhow::Error> {
    use windows::core::GUID;
    use windows::Win32::Foundation::S_OK;
    use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
    use windows::Win32::Media::Audio::*;
    use windows::Win32::System::Com::*;

    CoUninitialize();
    match CoInitialize(None) {
        S_OK => (),
        _ => Err(()),
    };

    let devicenum: IMMDeviceEnumerator =
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)?;

    let defaultdevice = devicenum.GetDefaultAudioEndpoint(eRender, eConsole)?;

    let endpointval: IAudioEndpointVolume = defaultdevice.Activate(CLSCTX_INPROC_SERVER, None)?;

    match vol {
        0.0 => endpointval.SetMute(true, 0 as *const GUID)?,
        _ => endpointval.SetMute(false, 0 as *const GUID)?,
    }

    endpointval.SetMasterVolumeLevelScalar(vol, 0 as *const GUID)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::downloader;
    use crate::sound::play_audio;
    use std::env;

    ///Tests that checks if audio plays correctly
    #[smol_potat::test]
    async fn test_play_audio() {
        let mut path = env::current_dir().unwrap();
        downloader::download_and_write("https://download.samplelib.com/mp3/sample-3s.mp3", &path)
            .await
            .unwrap();
        path.push("sample-3s.mp3");
        play_audio(path, 0.1).unwrap();
    }
}
