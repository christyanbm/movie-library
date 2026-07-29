mod db;
mod scanner;
mod tmdb;

use db::Movie;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    conn: Mutex<Connection>,
}

#[tauri::command]
fn scan_and_save(folder: String, state: State<AppState>) -> Result<usize, String> {
    let movies = scanner::scan_folder(&folder);
    let conn = state.conn.lock().map_err(|e| e.to_string())?;

    let mut count = 0;
    for movie in &movies {
        if db::insert_movie(&conn, movie).is_ok() {
            count += 1;
        }
    }
    Ok(count)
}

#[tauri::command]
fn get_movies(state: State<AppState>) -> Result<Vec<Movie>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    db::get_all_movies(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_progress(file_path: String, progress: i32, watched: bool, state: State<AppState>) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    db::update_progress(&conn, &file_path, progress, watched).map_err(|e| e.to_string())
}

#[tauri::command]
async fn fetch_metadata(movie_id: i64, title: String, year: Option<i32>, state: State<'_, AppState>) -> Result<(), String> {
    if let Some(result) = tmdb::search_movie(&title, year).await {
        let poster_url = result
            .poster_path
            .map(|p| tmdb::poster_full_url(&p))
            .unwrap_or_default();

        let conn = state.conn.lock().map_err(|e| e.to_string())?;
        db::update_tmdb_metadata(
            &conn,
            movie_id,
            result.id,
            &result.overview,
            &poster_url,
            result.vote_average,
            "", // los genre_ids necesitan otro llamado a /genre/movie/list para mapear nombres
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn main() {
    let conn = Connection::open("movie_library.db").expect("No se pudo abrir la base de datos");
    db::init_db(&conn).expect("No se pudo inicializar la base de datos");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState { conn: Mutex::new(conn) })
        .invoke_handler(tauri::generate_handler![
            scan_and_save,
            get_movies,
            save_progress,
            fetch_metadata
        ])
        .run(tauri::generate_context!())
        .expect("error corriendo la app de Tauri");
}
