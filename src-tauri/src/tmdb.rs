use serde::Deserialize;

// Consigue tu API key gratis en https://www.themoviedb.org/settings/api
const TMDB_API_KEY: &str = "4ff1e55e81b713206d950bb44f9c182a";
const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";
const TMDB_IMAGE_BASE: &str = "https://image.tmdb.org/t/p/w500";

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<TmdbMovie>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct TmdbMovie {
    pub id: i32,
    pub title: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub vote_average: f32,
    pub genre_ids: Vec<i32>,
}

pub async fn search_movie(title: &str, year: Option<i32>) -> Option<TmdbMovie> {
    let client = reqwest::Client::new();
    let mut url = format!(
        "{}/search/movie?api_key={}&query={}",
        TMDB_BASE_URL,
        TMDB_API_KEY,
        urlencoding::encode(title)
    );

    if let Some(y) = year {
        url.push_str(&format!("&year={}", y));
    }

    let resp = client.get(&url).send().await.ok()?;
    let parsed: SearchResponse = resp.json().await.ok()?;
    parsed.results.into_iter().next()
}

pub fn poster_full_url(poster_path: &str) -> String {
    format!("{}{}", TMDB_IMAGE_BASE, poster_path)
}
