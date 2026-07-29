mod db;
mod scanner;
mod tmdb;

use db::Movie;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    conn: Mutex<Connection>,
}

#[tauri::command]
fn scan_and_save(folder: String, state: State<AppState>) -> Result<usize, String> {
    let movies = scanner::scan_folder(&folder);
    let mut conn = state.conn.lock().map_err(|e| e.to_string())?;

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let mut count = 0;
    for movie in &movies {
        if db::insert_movie(&tx, movie).is_ok() {
            count += 1;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;
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
            "",
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn fetch_missing_metadata(state: State<'_, AppState>) -> Result<usize, String> {
    let movies = {
        let conn = state.conn.lock().map_err(|e| e.to_string())?;
        db::get_movies_without_poster(&conn).map_err(|e| e.to_string())?
    };

    let mut count = 0;
    for movie in &movies {
        if let Some(result) = tmdb::search_movie(&movie.title, movie.year).await {
            let poster_url = result
                .poster_path
                .map(|p| tmdb::poster_full_url(&p))
                .unwrap_or_default();

            if let Some(id) = movie.id {
                let conn = state.conn.lock().map_err(|e| e.to_string())?;
                db::update_tmdb_metadata(
                    &conn, id, result.id, &result.overview,
                    &poster_url, result.vote_average, "",
                )
                .map_err(|e| e.to_string())?;
            }
            count += 1;
        }
    }
    Ok(count)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_data = app.path().app_data_dir().expect("No se pudo obtener el directorio de datos");
            std::fs::create_dir_all(&app_data).expect("No se pudo crear el directorio de datos");
            let db_path = app_data.join("movie_library.db");
            let conn = Connection::open(&db_path).expect("No se pudo abrir la base de datos");
            db::init_db(&conn).expect("No se pudo inicializar la base de datos");
            app.manage(AppState { conn: Mutex::new(conn) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan_and_save,
            get_movies,
            save_progress,
            fetch_metadata,
            fetch_missing_metadata
        ])
        .run(tauri::generate_context!())
        .expect("error corriendo la app de Tauri");
}
