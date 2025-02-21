use std::process::{Command, Stdio};
use crate::error::AudioRustError;

pub fn convert_mp3_to_wav_in_memory(mp3_path: &str) -> Result<Vec<u8>, AudioRustError> {
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", mp3_path,
            "-f", "wav",
            "-ar", "16000",  // Tasa de muestreo requerida por Vosk
            "-ac", "1",     // Audio mono
            "-"
        ])
        .stdout(Stdio::piped())
        .output()
        .map_err(|e| AudioRustError::ConversionError(format!("Error al ejecutar ffmpeg: {}", e)))?;

    if !output.status.success() {
        return Err(AudioRustError::ConversionError(
            format!("Error al convertir MP3 a WAV: {}", String::from_utf8_lossy(&output.stderr))
        ));
    }

    Ok(output.stdout)
}