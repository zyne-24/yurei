use crate::types::{Action, VideoFormat, VideoItem};
use anyhow::Result;
use console::{style, Term};
use std::io::Write;
use std::process::{Command, Stdio};

pub fn show_header() -> Result<()> {
        let term = Term::stdout();
        term.clear_screen()?;
        println!("{}", style("========================================").cyan().bold());
        println!("{}", style("            RUST-TUBE CLI               ").cyan().bold());
        println!("{}", style("========================================").cyan().bold());
        Ok(())
}

pub fn get_user_input(prompt: &str) -> Result<String> {
        print!("{} ", style(prompt).bold().green());
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
}

pub fn select_video(videos: &[VideoItem]) -> Result<Option<VideoItem>> {
        let mut fzf_input = String::new();
        for v in videos {
                fzf_input.push_str(&format!("{} | {} | {} | {} | {}\n", 
                            v.title, v.channel, v.duration, v.id, v.thumbnail));
            }

        let mut fzf = Command::new("fzf")
                .args([
                        "--ansi",
                        "--delimiter", " \\| ",
                        "--with-nth", "1,2,3",
                        "--header", "Select Video",
                        "--layout", "reverse",
                        "--height", "100%",
                        "--preview", "chafa -f symbols --size=40x20 {5}",
                        "--preview-window", "right:50%",
                        "--pointer", "▶",
                    ])
                    .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

        fzf.stdin.as_mut().unwrap().write_all(fzf_input.as_bytes())?;
        
        let output = fzf.wait_with_output()?;
        let selected = String::from_utf8_lossy(&output.stdout);

        if selected.trim().is_empty() {
                return Ok(None);
            }

        let parts: Vec<&str> = selected.split('|').map(|s| s.trim()).collect();
        let selected_id = parts[3];

        Ok(videos.iter().find(|v| v.id == selected_id).cloned())
}

pub fn select_format(formats: &[VideoFormat]) -> Result<Option<VideoFormat>> {
        let mut fzf_input = String::new();
        for f in formats {
                fzf_input.push_str(&format!("{:<8} | {:<5} | {:<10} | {} | {}\n", 
                            f.resolution, f.ext, format!("{}fps", f.fps), f.vcodec, f.id));
            }

        let mut fzf = Command::new("fzf")
                .args([
                        "--height", "40%", 
                        "--layout", "reverse", 
                        "--header", "Select Resolution", 
                        "--delimiter", "\\|",
                        "--with-nth", "1,2,3,4"
                    ])
                    .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

        fzf.stdin.as_mut().unwrap().write_all(fzf_input.as_bytes())?;
        let output = fzf.wait_with_output()?;
        let selected = String::from_utf8_lossy(&output.stdout);

        if selected.trim().is_empty() {
                return Ok(None);
            }

        let parts: Vec<&str> = selected.split('|').map(|s| s.trim()).collect();
        let selected_id = parts[4];

        Ok(formats.iter().find(|f| f.id == selected_id).cloned())
}

pub fn select_action() -> Result<Option<Action>> {
        let mut fzf = Command::new("fzf")
                .args(["--height", "20%", "--layout", "reverse", "--header", "Action"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

        fzf.stdin.as_mut().unwrap().write_all(b"Stream\nDownload")?;
        let output = fzf.wait_with_output()?;
        let selected = String::from_utf8_lossy(&output.stdout);

        if selected.contains("Stream") {
                Ok(Some(Action::Stream))
            } else if selected.contains("Download") {
                Ok(Some(Action::Download))
            } else {
                Ok(None)
            }
}
