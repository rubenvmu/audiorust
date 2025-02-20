use std::process::{Command, Stdio};
use crate::error::AudioRustError;

pub fn convert_mp3_to_wav_in_memory(mp3_path: &str) -> Result<Vec<u8>, AudioRustError> {
    
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(mp3_path)
        .arg("-f")
        .arg("wav")
        .arg("-")
        .stdout(Stdio::piped())
        .output()
        .map_err(|e| AudioRustError::ConversionError(format!("Error al ejecutar ffmpeg: {}", e)))?;

    if !output.status.success() {
        return Err(AudioRustError::ConversionError("Error al convertir MP3 a WAV".to_string()));
    }

    Ok(output.stdout)
}