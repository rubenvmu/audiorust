use std::fmt;
use std::io;
use zip::result::ZipError;

#[derive(Debug)]
pub enum AudioRustError {
    ConversionError(String),
    TranscriptionError(String),
    ModelError(String),
    IoError(io::Error),
    ZipError(ZipError),
}

impl fmt::Display for AudioRustError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioRustError::ConversionError(msg) => write!(f, "Error de conversión: {}", msg),
            AudioRustError::TranscriptionError(msg) => write!(f, "Error de transcripción: {}", msg),
            AudioRustError::ModelError(msg) => write!(f, "Error del modelo: {}", msg),
            AudioRustError::IoError(err) => write!(f, "Error de I/O: {}", err),
            AudioRustError::ZipError(err) => write!(f, "Error de ZIP: {}", err),
        }
    }
}

impl From<io::Error> for AudioRustError {
    fn from(err: io::Error) -> Self {
        AudioRustError::IoError(err)
    }
}

impl From<ZipError> for AudioRustError {
    fn from(err: ZipError) -> Self {
        AudioRustError::ZipError(err)
    }
}