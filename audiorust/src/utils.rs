use std::fs::File;
use std::io::{Read, Write};
use reqwest::blocking::get;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use zip::ZipArchive;

const MODEL_URL: &str = "https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip";
const MODEL_DIR: &str = "model";

pub fn ensure_model_exists() -> Result<(), crate::error::AudioRustError> {
    if !Path::new(MODEL_DIR).exists() {
        println!("Descargando modelo de Vosk...");

        // Descargar el modelo
        let response = get(MODEL_URL).map_err(|e| {
            crate::error::AudioRustError::ModelError(format!("Error al descargar el modelo: {}", e))
        })?;

        let total_size = response.content_length().unwrap_or(0);

        // Configurar la barra de progreso
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        // Crear el archivo ZIP
        let mut dest = File::create("model.zip")?;

        // Escribir el contenido descargado en el archivo ZIP
        let mut content = response;
        let mut buffer = [0; 8192];
        loop {
            let n = content.read(&mut buffer)?;
            if n == 0 { break; }
            dest.write_all(&buffer[..n])?;
            pb.inc(n as u64);
        }

        pb.finish_with_message("Descarga completada");

        println!("Descomprimiendo el modelo...");

        // Descomprimir el archivo ZIP
        let file = File::open("model.zip")?;
        let mut archive = ZipArchive::new(file)?;
        archive.extract(MODEL_DIR)?;

        // Eliminar el archivo ZIP después de descomprimirlo
        std::fs::remove_file("model.zip")?;
    }

    Ok(())
}