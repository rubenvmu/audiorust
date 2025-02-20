mod audio;
mod transcriber;
mod utils;
mod error;

use std::env;
use crate::error::AudioRustError;

fn main() -> Result<(), AudioRustError> {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} <archivo_mp3>", args[0]);
        return Ok(());
    }

    let mp3_path = &args[1];

    
    utils::ensure_model_exists()?;

    
    let audio_data = audio::convert_mp3_to_wav_in_memory(mp3_path)?;

    
    let text = transcriber::transcribe_audio_to_text_from_memory(&audio_data)?;
    println!("Texto transcrito:\n{}", text);

    Ok(())
}