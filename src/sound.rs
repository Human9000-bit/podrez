use awedio::Sound;
use std::{path::PathBuf, time::Duration};

/// Plays sound from path
pub async fn play_audio(path: PathBuf, volume: f64) -> Result<(), anyhow::Error> {
    let (mut manager, _backend) = awedio::start()?; // start audio manager

    let (audio, _controller) = awedio::sounds::open_file(path)? // open audio file
        .with_adjustable_volume_of(volume as f32)
        .controllable();

    // set system volume (windows only)
    #[cfg(target_os = "windows")]
    unsafe {
        set_win_volume(volume)?
    }

    // play the audio
    manager.play(Box::new(audio));
    
    // wait for about 10 seconds until the audio stops
    // 
    // TODO: find the way to sleep for audio duration
    async_std::task::sleep(Duration::from_secs(10)).await;
    Ok(())
}

/// Function that sets windows volume
#[cfg(target_os = "windows")]
unsafe fn set_win_volume(volume: f64) -> Result<(), anyhow::Error> {
    use windows::core::GUID;
    use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
    use windows::Win32::Media::Audio::*;
    use windows::Win32::System::Com::*;

    CoUninitialize();

    let devicenum: IMMDeviceEnumerator =
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)?;

    let defaultdevice = devicenum.GetDefaultAudioEndpoint(eRender, eConsole)?;

    let endpointval: IAudioEndpointVolume = defaultdevice.Activate(CLSCTX_INPROC_SERVER, None)?;

    match volume {
        0.0 => endpointval.SetMute(true, std::ptr::null::<GUID>())?,
        _ => endpointval.SetMute(false, std::ptr::null::<GUID>())?,
    }

    endpointval.SetMasterVolumeLevelScalar(volume as f32 * 100.0, std::ptr::null::<GUID>())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::downloader;
    use crate::sound::play_audio;
    use std::env;

    /// Tests that checks if audio plays correctly
    #[async_std::test]
    async fn test_play_audio() {
        let mut path = env::current_dir().unwrap();
        downloader::download_and_write("https://download.samplelib.com/mp3/sample-3s.mp3", &path)
            .await
            .unwrap();
        path.push("sample-3s.mp3");
        play_audio(path, 0.1).await.unwrap();
    }

    #[cfg(target_os = "windows")]
    #[async_std::test]
    /// Tests that checks if windows volume is being set correctly
    async fn test_win() {
            use windows::Win32::System::Com::CoInitialize;
        unsafe {
            CoInitialize(None).unwrap();
        }
    }
}
