use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]  
pub enum AudioRustError {
    ConversionError(String),
    TranscriptionError(String),
    ModelError(String),
    IoError(std::io::Error),
    ApiError(String),  
}

impl fmt::Display for AudioRustError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioRustError::ConversionError(msg) => write!(f, "Error de conversión: {}", msg),
            AudioRustError::TranscriptionError(msg) => write!(f, "Error de transcripción: {}", msg),
            AudioRustError::ModelError(msg) => write!(f, "Error del modelo: {}", msg),
            AudioRustError::IoError(err) => write!(f, "Error de I/O: {}", err),
            AudioRustError::ApiError(msg) => write!(f, "Error de API: {}", msg),  
        }
    }
}