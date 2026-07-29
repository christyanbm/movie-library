use crate::db::Movie;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

const VIDEO_EXTENSIONS: [&str; 6] = ["mp4", "mkv", "avi", "mov", "wmv", "m4v"];

/// Recorre una carpeta recursivamente y devuelve un Movie "crudo" por cada video encontrado
pub fn scan_folder(folder: &str) -> Vec<Movie> {
    let mut movies = Vec::new();

    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if VIDEO_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                    if let Some(movie) = build_movie_from_path(path) {
                        movies.push(movie);
                    }
                }
            }
        }
    }
    movies
}

fn build_movie_from_path(path: &Path) -> Option<Movie> {
    let file_name = path.file_name()?.to_str()?.to_string();
    let (title, year) = parse_title_year(&file_name);
    let size_bytes = std::fs::metadata(path).ok().map(|m| m.len() as i64);
    let (duration, resolution, codec) = probe_video(path);

    Some(Movie {
        id: None,
        file_path: path.to_str()?.to_string(),
        file_name,
        title,
        year,
        duration_seconds: duration,
        resolution,
        codec,
        size_bytes,
        tmdb_id: None,
        overview: None,
        poster_url: None,
        rating: None,
        genres: None,
        watched: false,
        progress_seconds: Some(0),
    })
}

/// Extrae título y año de nombres tipo:
/// "Interstellar.2014.1080p.BluRay.x264-GROUP.mkv" -> ("Interstellar", Some(2014))
fn parse_title_year(file_name: &str) -> (String, Option<i32>) {
    let stem = Path::new(file_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_name);

    // separadores comunes: puntos, guiones bajos
    let normalized = stem.replace('.', " ").replace('_', " ");

    // buscar un año de 4 dígitos (19xx / 20xx)
    let words: Vec<&str> = normalized.split_whitespace().collect();
    let mut year: Option<i32> = None;
    let mut cut_index = words.len();

    for (i, w) in words.iter().enumerate() {
        if w.len() == 4 {
            if let Ok(y) = w.parse::<i32>() {
                if (1900..=2100).contains(&y) {
                    year = Some(y);
                    cut_index = i;
                    break;
                }
            }
        }
    }

    let title = if cut_index > 0 {
        words[..cut_index].join(" ")
    } else {
        normalized.clone()
    };

    (title.trim().to_string(), year)
}

/// Llama a ffprobe (debe estar instalado en el sistema) y extrae duración/resolución/codec
fn probe_video(path: &Path) -> (Option<i32>, Option<String>, Option<String>) {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=width,height,codec_name:format=duration",
            "-of", "json",
        ])
        .arg(path)
        .output();

    let output = match output {
        Ok(o) => o,
        Err(_) => return (None, None, None), // ffprobe no instalado o falló
    };

    let json_str = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap_or_default();

    let duration = parsed["format"]["duration"]
        .as_str()
        .and_then(|d| d.parse::<f64>().ok())
        .map(|d| d as i32);

    let (width, height, codec) = if let Some(stream) = parsed["streams"].get(0) {
        (
            stream["width"].as_i64(),
            stream["height"].as_i64(),
            stream["codec_name"].as_str().map(|s| s.to_string()),
        )
    } else {
        (None, None, None)
    };

    let resolution = match (width, height) {
        (Some(w), Some(h)) => Some(format!("{}x{}", w, h)),
        _ => None,
    };

    (duration, resolution, codec)
}
