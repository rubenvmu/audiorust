# AudioRust 🎵

**AudioRust** es una herramienta escrita en Rust que permite convertir archivos de audio MP3 a WAV y transcribir el contenido de los archivos de audio mediante la API de Vosk. Este proyecto está diseñado para ser eficiente, extensible y fácil de integrar en otras aplicaciones que requieran funcionalidades de procesamiento de audio y transcripción.

## Características ✨

- **Conversión de MP3 a WAV**: Utiliza `ffmpeg` para convertir archivos MP3 en formato WAV de manera rápida y eficiente.
- **Transcripción de Audio**: Utiliza el motor de reconocimiento de voz de Vosk para transcribir audio en tiempo real desde archivos WAV.
- **Manejo de Errores**: Implementación robusta de manejo de errores mediante un sistema propio (`AudioRustError`), lo que garantiza una experiencia más controlada y predecible al ejecutar operaciones de conversión y transcripción.
- **Eficiencia y Escalabilidad**: Diseñado con un enfoque en la eficiencia para trabajar con archivos grandes y realizar tareas de transcripción de manera escalable.

## Requisitos 📋

- **Rust 1.XX o superior**.
- **FFmpeg**: Asegúrate de tener `ffmpeg` instalado en tu sistema para la conversión de archivos MP3 a WAV.
- **API de Vosk**: Este proyecto hace uso de la API de Vosk para la transcripción de audio. Asegúrate de tener un servidor Vosk corriendo localmente.

### Instalar FFmpeg

- **En sistemas basados en Debian/Ubuntu**:
  ```bash
  sudo apt-get install ffmpeg
  ```

- **En macOS (usando Homebrew)**:
  ```bash
  brew install ffmpeg
  ```

### Ejecutar servidor de Vosk

Para ejecutar un servidor local de Vosk, sigue las instrucciones oficiales del proyecto de Vosk para descargar y configurar el modelo. Asegúrate de que el servidor esté corriendo en la dirección `http://localhost:2700`.

## Instalación 🛠️

### Clonar el repositorio

Primero, clona el repositorio en tu máquina local:

```bash
git clone https://github.com/tu-usuario/audio-rust.git
cd audio-rust
```

### Instalar dependencias

Asegúrate de tener Rust instalado y actualizado. Luego, instala las dependencias del proyecto con:

```bash
cargo build
```

## Uso 🚀

### Convertir MP3 a WAV

Para convertir un archivo MP3 a WAV, ejecuta el siguiente comando en tu terminal, asegurándote de reemplazar las rutas a los archivos de entrada y salida:

```bash
cargo run --convert_mp3_to_wav /ruta/al/archivo.mp3 /ruta/de/salida/archivo.wav
```

### Transcripción de Audio

Para transcribir un archivo de audio a texto, usa el siguiente comando. Asegúrate de tener un archivo WAV listo y que el servidor Vosk esté ejecutándose:

```bash
cargo run --transcribe_audio /ruta/al/archivo.wav
```

Esto enviará el archivo WAV a la API de Vosk y devolverá el texto transcrito.

## Licencia 📄

Este proyecto está bajo la Licencia MIT. Para más detalles, consulta el archivo [LICENSE](LICENSE).
