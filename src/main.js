import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { convertFileSrc } from "@tauri-apps/api/core";

const grid = document.getElementById("grid");
const scanBtn = document.getElementById("scanBtn");
const player = document.getElementById("player");
const videoEl = document.getElementById("videoEl");
const closePlayer = document.getElementById("closePlayer");

async function loadMovies() {
  const movies = await invoke("get_movies");
  renderGrid(movies);
}

function renderGrid(movies) {
  grid.innerHTML = "";
  for (const movie of movies) {
    const card = document.createElement("div");
    card.className = "card";
    card.innerHTML = `
      <div class="poster" style="background-image: url('${movie.poster_url || ""}')">
        ${movie.watched ? '<span class="badge">✔ Vista</span>' : ""}
      </div>
      <div class="info">
        <h3>${movie.title}</h3>
        <p>${movie.year ?? ""} · ${movie.resolution ?? "?"}</p>
      </div>
    `;
    card.addEventListener("click", () => playMovie(movie));
    grid.appendChild(card);
  }
}

function playMovie(movie) {
  const src = convertFileSrc(movie.file_path);
  videoEl.src = src;
  videoEl.currentTime = movie.progress_seconds || 0;
  player.classList.remove("hidden");
  videoEl.play();

  // guardar progreso cada 5 segundos
  const interval = setInterval(() => {
    invoke("save_progress", {
      filePath: movie.file_path,
      progress: Math.floor(videoEl.currentTime),
      watched: videoEl.currentTime / videoEl.duration > 0.9,
    });
  }, 5000);

  videoEl.addEventListener("ended", () => clearInterval(interval), { once: true });
  closePlayer.addEventListener(
    "click",
    () => {
      clearInterval(interval);
      videoEl.pause();
      player.classList.add("hidden");
    },
    { once: true }
  );
}

scanBtn.addEventListener("click", async () => {
  const folder = await open({ directory: true, multiple: false });
  if (!folder) return;

  scanBtn.textContent = "Escaneando...";
  const count = await invoke("scan_and_save", { folder });
  scanBtn.textContent = "Escanear carpeta";
  alert(`Se agregaron ${count} películas nuevas`);
  loadMovies();
});

loadMovies();
