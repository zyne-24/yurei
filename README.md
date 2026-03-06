# YUREI - Youtube CLI
**YUREI** is a lightweight, interactive YouTube CLI written in Rust. It allows you to search, stream, and download videos directly from your terminal using the power of yt-dlp and fzf.
## 🚀 Installation
#### Via Cargo
```bash
cargo install --git https://github.com/zyne-24/yurei
```

#### Manual Install
```bash
git clone https://github.com/zyne-24/yurei && cd yurei && cargo build --release && sudo cp target/release/yurei /usr/local/bin/
```
> [!IMPORTANT]
> **Dependencies:** Ensure [yt-dlp](https://github.com/yt-dlp/yt-dlp), [fzf](https://github.com/junegunn/fzf), [mpv](https://github.com/mpv-player/mpv), and [chafa](https://github.com/hpjansson/chafa) are installed on your system.

## ⌨️ Usage
```bash
yurei
```

Search directly for a video
```bash
yurei <query>
```

## 🔍 Structure
 * `main.rs`: Core logic; manages program flow, CLI arguments, and pagination.
 * `youtube.rs`: Data fetcher; interfaces with yt-dlp to extract video metadata and formats.
 * `ui.rs`: Interface layer; handles fzf menus and terminal thumbnail previews.
 * `runner.rs`: Executor; triggers mpv for streaming and yt-dlp for downloading.
 * `types.rs`: Blueprints; defines shared structs and enums for data consistency.
## Modification
 * High-Res Previews: For terminals supporting `Kitty` or `Sixel` protocols, replace the `chafa` command in `ui.rs` with `icat` or `chafa --format=sixel` for sharper thumbnails.
 * Audio-Only Mode: Add an `--audio` flag in `main.rs` and pass the `--no-video` argument to `mpv` in `runner.rs`.
 * Result Caching: Store search results in memory or a temp file to make "Previous Page" navigation instant without re-fetching.
 * Custom FZF Bindings: Use `--bind` flags in `ui.rs` to create shortcuts (e.g., CTRL+S to stream immediately).
 * Async Fetching: Use `tokio::process` to keep the UI responsive during heavy metadata extraction.
   
---

## 🌟 Inspired by

**YUREI** is built with the same spirit of performance and simplicity found in these projects:

* [**animeku-cli**](https://github.com/lucasbuilds/animeku-cli) - For demonstrating how **Rust** can create a clean, incredibly fast streaming experience in the terminal.
* [**yt-cli**](https://github.com/BishrGhalil/yt-cli) - For the initial concept of interacting with YouTube via command line.
* [**yt-fzf**](https://github.com/pystardust/ytfzf) - For the powerful logic of combining `yt-dlp` and `fzf`.

---
