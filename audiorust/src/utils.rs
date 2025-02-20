use std::fs::File;
use std::io::{Read, Write};
use reqwest::blocking::get;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

const MODEL_URL: &str = "https://example.com/model.zip";
const MODEL_DIR: &str = "model";

pub fn ensure_model_exists() -> Result<(), crate::error::AudioRustError> {
    if !Path::new(MODEL_DIR).exists() {
        println!("Descargando modelo de Vosk...");

        let response = get(MODEL_URL).map_err(|e| {
            crate::error::AudioRustError::ModelError(format!("Error al descargar el modelo: {}", e))
        })?;

        let total_size = response.content_length().ok_or_else(|| {
            crate::error::AudioRustError::ModelError("No se pudo obtener el tamaño del archivo".to_string())
        })?;

        let pb = ProgressBar::new(total_size);

        
        let style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .map_err(|e| {
                crate::error::AudioRustError::ModelError(format!("Error al crear el estilo de la barra de progreso: {}", e))
            })?
            .progress_chars("#>-");

        pb.set_style(style);

        let mut dest = File::create("model.zip").map_err(|e| {
            crate::error::AudioRustError::ModelError(format!("Error al crear el archivo: {}", e))
        })?;

        let mut content = response;
        let mut buffer = [0; 8192];
        while let Ok(n) = content.read(&mut buffer) {
            if n == 0 {
                break;
            }
            dest.write_all(&buffer[..n]).map_err(|e| {
                crate::error::AudioRustError::ModelError(format!("Error al escribir el archivo: {}", e))
            })?;
            pb.inc(n as u64);
        }

        pb.finish_with_message("Descarga completada");

        
        println!("Descomprimiendo el modelo...");
        
    }

    Ok(())
}