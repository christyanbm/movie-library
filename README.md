# Movie Library — Esqueleto Tauri

Proyecto base para tu reproductor/organizador de peliculas de escritorio.

## Requisitos previos

1. **Rust** — https://www.rust-lang.org/tools/install
2. **Node.js** (18+) — https://nodejs.org
3. **ffmpeg/ffprobe** instalado y en el PATH del sistema (necesario para leer duración/resolución):
   - Windows: descarga desde https://ffmpeg.org/download.html y agrega la carpeta `bin` al PATH
   - Mac: `brew install ffmpeg`
   - Linux: `sudo apt install ffmpeg`
4. Dependencias del sistema para Tauri: sigue la guía oficial según tu SO → https://v2.tauri.app/start/prerequisites/

## Instalación

```bash
npm install
npm install @tauri-apps/plugin-dialog
cargo add tauri-plugin-dialog --manifest-path src-tauri/Cargo.toml
```

## Configurar tu TMDb API Key

1. Crea una cuenta gratis en https://www.themoviedb.org
2. Ve a Configuración → API y genera una API key
3. Pégala en `src-tauri/src/tmdb.rs` reemplazando `TU_API_KEY_AQUI`

## Correr en modo desarrollo

```bash
npm run tauri dev
```

## Compilar el ejecutable final

```bash
npm run tauri build
```

## Estructura del proyecto

```
movie-library/
├── src/                  # Frontend (HTML/CSS/JS)
│   ├── index.html
│   ├── main.js           # Lógica: grid, reproductor, llamadas a Rust
│   └── style.css
├── src-tauri/
│   ├── src/
│   │   ├── main.rs       # Comandos expuestos al frontend
│   │   ├── db.rs         # SQLite: esquema y queries
│   │   ├── scanner.rs    # Escaneo de carpetas + ffprobe + parseo de nombres
│   │   └── tmdb.rs       # Cliente de la API de TMDb
│   ├── Cargo.toml
│   └── tauri.conf.json
└── package.json
```

## Qué ya funciona (esqueleto)

- ✅ Escanear una carpeta recursivamente y detectar videos
- ✅ Parsear título/año del nombre del archivo
- ✅ Extraer duración/resolución/codec con ffprobe
- ✅ Guardar todo en SQLite (`movie_library.db`, se crea junto al ejecutable)
- ✅ Grid visual tipo Netflix
- ✅ Reproductor con `<video>` HTML5 + guardado de progreso cada 5s
- ✅ Búsqueda de metadata en TMDb (falta conectar el botón en el frontend)

## Siguientes pasos sugeridos

1. Agregar un botón "Buscar metadata" por película que llame a `fetch_metadata`
2. Agregar filtros/búsqueda en el frontend (por género, año, vista/no vista)
3. Si algún video no reproduce bien con `<video>` HTML5 (códecs raros como HEVC en mkv),
   cambiar el reproductor a **libmpv** vía el crate `libmpv-rs`, embebido en una ventana nativa
4. Detección de duplicados: comparar tamaño de archivo + título parseado
5. Soporte de subtítulos: buscar archivos `.srt` junto al video y pasarlos como `<track>` en el `<video>`

## Notas

- La base de datos SQLite vive junto al binario compilado. Si quieres reubicarla,
  cambia la ruta en `main.rs`, línea `Connection::open("movie_library.db")`.
- El escaneo es síncrono por ahora; si tienes muchísimos archivos, conviene
  moverlo a un hilo/tarea async con reporte de progreso al frontend.
