use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Movie {
    pub id: Option<i64>,
    pub file_path: String,
    pub file_name: String,
    pub title: String,
    pub year: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub resolution: Option<String>,
    pub codec: Option<String>,
    pub size_bytes: Option<i64>,
    // metadata online (TMDb)
    pub tmdb_id: Option<i32>,
    pub overview: Option<String>,
    pub poster_url: Option<String>,
    pub rating: Option<f32>,
    pub genres: Option<String>, // guardado como "Accion,Drama"
    // progreso de reproducción
    pub watched: bool,
    pub progress_seconds: Option<i32>,
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS movies (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            file_path       TEXT NOT NULL UNIQUE,
            file_name       TEXT NOT NULL,
            title           TEXT NOT NULL,
            year            INTEGER,
            duration_seconds INTEGER,
            resolution      TEXT,
            codec           TEXT,
            size_bytes      INTEGER,
            tmdb_id         INTEGER,
            overview        TEXT,
            poster_url      TEXT,
            rating          REAL,
            genres          TEXT,
            watched         INTEGER DEFAULT 0,
            progress_seconds INTEGER DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_movie(conn: &Connection, movie: &Movie) -> Result<i64> {
    conn.execute(
        "INSERT OR IGNORE INTO movies
        (file_path, file_name, title, year, duration_seconds, resolution, codec, size_bytes)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            movie.file_path,
            movie.file_name,
            movie.title,
            movie.year,
            movie.duration_seconds,
            movie.resolution,
            movie.codec,
            movie.size_bytes,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_movies_without_poster(conn: &Connection) -> Result<Vec<Movie>> {
    let mut stmt = conn.prepare("SELECT * FROM movies WHERE poster_url IS NULL OR poster_url = '' ORDER BY title ASC")?;
    let rows = stmt.query_map([], map_movie_row)?;
    rows.collect()
}

fn map_movie_row(row: &rusqlite::Row) -> rusqlite::Result<Movie> {
    Ok(Movie {
        id: row.get(0)?,
        file_path: row.get(1)?,
        file_name: row.get(2)?,
        title: row.get(3)?,
        year: row.get(4)?,
        duration_seconds: row.get(5)?,
        resolution: row.get(6)?,
        codec: row.get(7)?,
        size_bytes: row.get(8)?,
        tmdb_id: row.get(9)?,
        overview: row.get(10)?,
        poster_url: row.get(11)?,
        rating: row.get(12)?,
        genres: row.get(13)?,
        watched: row.get::<_, i32>(14)? != 0,
        progress_seconds: row.get(15)?,
    })
}

pub fn get_all_movies(conn: &Connection) -> Result<Vec<Movie>> {
    let mut stmt = conn.prepare("SELECT * FROM movies ORDER BY title ASC")?;
    let rows = stmt.query_map([], map_movie_row)?;
    rows.collect()
}

pub fn update_progress(conn: &Connection, file_path: &str, progress: i32, watched: bool) -> Result<()> {
    conn.execute(
        "UPDATE movies SET progress_seconds = ?1, watched = ?2 WHERE file_path = ?3",
        params![progress, watched as i32, file_path],
    )?;
    Ok(())
}

pub fn update_tmdb_metadata(
    conn: &Connection,
    id: i64,
    tmdb_id: i32,
    overview: &str,
    poster_url: &str,
    rating: f32,
    genres: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE movies SET tmdb_id = ?1, overview = ?2, poster_url = ?3, rating = ?4, genres = ?5 WHERE id = ?6",
        params![tmdb_id, overview, poster_url, rating, genres, id],
    )?;
    Ok(())
}
