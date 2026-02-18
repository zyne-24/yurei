use crate::types::{VideoFormat, VideoItem};
use anyhow::Result;
use serde_json::Value;
use std::process::Command;

pub fn search(query: &str, page: u64) -> Result<Vec<VideoItem>> {
        let page_size = 10;
        let fetch_limit = page * page_size; 
        let playlist_start = (page - 1) * page_size + 1;
        let playlist_end = page * page_size;

        let output = Command::new("yt-dlp")
                .args([
                        format!("ytsearch{}:{}", fetch_limit, query),
                        "--dump-json".into(),
                        "--flat-playlist".into(),
                        "--no-warnings".into(),
                        "--playlist-start".into(), playlist_start.to_string(),
                        "--playlist-end".into(), playlist_end.to_string(),
                    ])
                    .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut videos = Vec::new();

        for line in stdout.lines() {
                if let Ok(v) = serde_json::from_str::<Value>(line) {
                        let id = v["id"].as_str().unwrap_or("").to_string();
                        if id.is_empty() { continue; }

                        videos.push(VideoItem {
                                title: v["title"].as_str().unwrap_or("?").to_string(),
                                channel: v["uploader"].as_str().unwrap_or("Unknown").to_string(),
                                duration: v["duration_string"].as_str().unwrap_or("??:??").to_string(),
                                thumbnail: format!("https://i.ytimg.com/vi/{}/hqdefault.jpg", id),
                                id,
                            });
                    }
            }
        Ok(videos)
}

pub fn get_formats(url: &str) -> Result<Vec<VideoFormat>> {
        let output = Command::new("yt-dlp")
                .args(["-J", url])
                .output()?;

        let json: Value = serde_json::from_slice(&output.stdout)?;
        let mut formats = Vec::new();
        let mut unique_keys = Vec::new();

        if let Some(raw_formats) = json["formats"].as_array() {
                for f in raw_formats.iter().rev() {
                        let vcodec = f["vcodec"].as_str().unwrap_or("none");
                        let height = f["height"].as_u64().unwrap_or(0);

                        if vcodec != "none" && height > 0 {
                                let ext = f["ext"].as_str().unwrap_or("mp4").to_string();
                                let resolution = format!("{}p", height);
                                let key = format!("{}-{}", resolution, ext);

                                if !unique_keys.contains(&key) {
                                        unique_keys.push(key);
                                        formats.push(VideoFormat {
                                                resolution,
                                                ext,
                                                fps: f["fps"].as_u64().unwrap_or(30),
                                                id: f["format_id"].as_str().unwrap_or("").to_string(),
                                                vcodec: vcodec.to_string(),
                                            });
                                    }
                            }
                    }
            }
        Ok(formats)
}
