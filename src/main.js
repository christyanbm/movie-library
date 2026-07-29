import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { convertFileSrc } from "@tauri-apps/api/core";

const grid = document.getElementById("grid");
const scanBtn = document.getElementById("scanBtn");
const player = document.getElementById("player");
const videoEl = document.getElementById("videoEl");
const closePlayer = document.getElementById("closePlayer");

let progressInterval = null;
let currentMoviePath = null;

async function loadMovies() {
  const movies = await invoke("get_movies");
  renderGrid(movies);
  invoke("fetch_missing_metadata").then(count => {
    if (count > 0) loadMovies();
  });
}

function renderGrid(movies) {
  grid.innerHTML = "";
  for (const movie of movies) {
    const card = document.createElement("div");
    card.className = "card";

    const poster = document.createElement("div");
    poster.className = "poster";
    if (movie.poster_url) {
      poster.style.backgroundImage = `url('${movie.poster_url}')`;
    }
    if (movie.watched) {
      const badge = document.createElement("span");
      badge.className = "badge";
      badge.textContent = "\u2714 Vista";
      poster.appendChild(badge);
    }

    const info = document.createElement("div");
    info.className = "info";

    const title = document.createElement("h3");
    title.textContent = movie.title;

    const detail = document.createElement("p");
    detail.textContent = `${movie.year ?? ""} \u00B7 ${movie.resolution ?? "?"}`;

    info.appendChild(title);
    info.appendChild(detail);
    card.appendChild(poster);
    card.appendChild(info);
    card.addEventListener("click", () => playMovie(movie));
    grid.appendChild(card);
  }
}

function playMovie(movie) {
  if (progressInterval) clearInterval(progressInterval);

  const src = convertFileSrc(movie.file_path);
  videoEl.src = src;
  videoEl.currentTime = movie.progress_seconds || 0;
  currentMoviePath = movie.file_path;
  player.classList.remove("hidden");
  videoEl.play();

  progressInterval = setInterval(() => {
    invoke("save_progress", {
      filePath: currentMoviePath,
      progress: Math.floor(videoEl.currentTime),
      watched: videoEl.currentTime / videoEl.duration > 0.9,
    });
  }, 5000);
}

closePlayer.addEventListener("click", () => {
  if (progressInterval) clearInterval(progressInterval);
  videoEl.pause();
  player.classList.add("hidden");
});

videoEl.addEventListener("ended", () => {
  if (progressInterval) clearInterval(progressInterval);
  player.classList.add("hidden");
});

scanBtn.addEventListener("click", async () => {
  const folder = await open({ directory: true, multiple: false });
  if (!folder) return;

  scanBtn.textContent = "Escaneando...";
  const count = await invoke("scan_and_save", { folder });
  scanBtn.textContent = "Escanear carpeta";
  alert(`Se agregaron ${count} pel\u00EDculas nuevas`);
  await loadMovies();
  const updated = await invoke("fetch_missing_metadata");
  if (updated > 0) loadMovies();
});

loadMovies();
