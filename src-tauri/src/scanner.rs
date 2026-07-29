use crate::db::Movie;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

const VIDEO_EXTENSIONS: [&str; 6] = ["mp4", "mkv", "avi", "mov", "wmv", "m4v"];

pub fn scan_folder(folder: &str) -> Vec<Movie> {
    let mut movies = Vec::new();

    for entry in WalkDir::new(folder).follow_links(true).into_iter().filter_map(|e| e.ok()) {
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

fn parse_title_year(file_name: &str) -> (String, Option<i32>) {
    let stem = Path::new(file_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_name);

    let normalized = stem.replace('.', " ").replace('_', " ");
    let words: Vec<&str> = normalized.split_whitespace().collect();

    let mut year: Option<i32> = None;
    let mut cut_index: Option<usize> = None;

    for (i, w) in words.iter().enumerate().rev() {
        if w.len() == 4 {
            if let Ok(y) = w.parse::<i32>() {
                if (1900..=2100).contains(&y) {
                    year = Some(y);
                    cut_index = Some(i);
                    break;
                }
            }
        }
    }

    let title = match cut_index {
        Some(idx) if idx > 0 => words[..idx].join(" "),
        _ => normalized.clone(),
    };

    (title.trim().to_string(), year)
}

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
        Err(_) => {
            eprintln!("ADVERTENCIA: ffprobe no encontrado. Instala ffmpeg/ffprobe para obtener metadatos de video.");
            return (None, None, None);
        }
    };

    let json_str = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = match serde_json::from_str(&json_str) {
        Ok(v) => v,
        Err(_) => return (None, None, None),
    };

    let duration = parsed["format"]["duration"]
        .as_str()
        .and_then(|d| d.parse::<f64>().ok())
        .map(|d| d.round() as i32);

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
