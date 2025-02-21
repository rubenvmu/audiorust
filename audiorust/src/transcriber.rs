use vosk::{Model, Recognizer};
use crate::error::AudioRustError;

pub fn transcribe_audio_to_text_from_memory(audio_data: &[u8]) -> Result<String, AudioRustError> {
    // Cargar modelo desde el directorio
    let model = Model::new("model")
        .ok_or_else(|| AudioRustError::ModelError("Error cargando modelo".to_string()))?;

    // Crear reconocedor
    let mut recognizer = Recognizer::new(&model, 16000.0)
        .ok_or_else(|| AudioRustError::TranscriptionError("Error creando reconocedor".to_string()))?;

    // Convertir audio a formato i16 (little-endian)
    let audio_data_i16: Vec<i16> = audio_data.chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    // Procesar audio
    recognizer.accept_waveform(&audio_data_i16)
        .map_err(|e| AudioRustError::TranscriptionError(format!("Error procesando audio: {:?}", e)))?;

    // Obtener el resultado final
    let result = match recognizer.final_result() {
        vosk::CompleteResult::Single(single) => single.text.to_string(),
        vosk::CompleteResult::Multiple(multiple) => {
            // Tomar el texto de la primera alternativa (la más probable)
            multiple.alternatives.first()
                .map(|alt| alt.text.to_string())
                .unwrap_or_else(|| "".to_string())
        }
    };

    Ok(result)
}