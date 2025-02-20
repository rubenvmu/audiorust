use reqwest::blocking::Client;
use crate::error::AudioRustError;
use serde_json::Value;

pub fn transcribe_audio_to_text_from_memory(audio_data: &[u8]) -> Result<String, AudioRustError> {
    
    let url = "http://example.com/api/transcribe";
    let client = Client::new();
    
    
    let response = client
        .post(url)
        .header("Content-Type", "audio/wav")
        .body(audio_data.to_vec())
        .send()
        .map_err(|e| AudioRustError::ApiError(format!("Error al enviar solicitud: {}", e)))?;

    
    if !response.status().is_success() {
        return Err(AudioRustError::ApiError(format!(
            "Error en la respuesta de la API: {}",
            response.status()
        )));
    }

    
    let result: Value = response.json().map_err(|e| AudioRustError::ApiError(format!(
        "Error al deserializar la respuesta JSON: {}", e
    )))?;

    
    let text = result["text"]
        .as_str()
        .ok_or_else(|| AudioRustError::ApiError("Respuesta sin texto transcrito".into()))?;

    Ok(text.to_string())
}
