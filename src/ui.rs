use crate::types::{Action, Selection, VideoFormat, VideoItem};
use anyhow::Result;
use console::{style, Term};
use std::io::Write;
use std::process::{Command, Stdio};

pub fn show_header() -> Result<()> {
        let term = Term::stdout();
        term.clear_screen()?;
        println!("{}", style("========================================").cyan().bold());
        println!("{}", style("            YUREI - YT CLI              ").cyan().bold());
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

pub fn select_video(videos: &[VideoItem], current_page: u64) -> Result<Selection> {
        let mut fzf_input = String::new();

        for v in videos {
                fzf_input.push_str(&format!("{} │ {} │ {} │ {} │ {}\n", 
                            v.title, v.channel, v.duration, v.id, v.thumbnail));
            }

        fzf_input.push_str("➡️  NEXT PAGE │ Nav │ - │ next │ -\n");
        if current_page > 1 {
                fzf_input.push_str("⬅️  PREV PAGE │ Nav │ - │ prev │ -\n");
        }
    fzf_input.push_str("❌ QUIT │ Exit │ - │ quit │ -\n");

        let mut fzf = Command::new("fzf")
                .args([
                        "--ansi",
                        "--delimiter", " │ ",
                        "--with-nth", "1,2,3",
                        "--header", &format!("Page {} (Enhanced Preview)", current_page),
                        "--layout", "reverse",
                        "--height", "100%",
                        "--cycle",
                        "--preview", "curl -sL {5} | chafa --format=symbols --symbols=all --colors=full --size=60x30 --stretch -", 
                        "--preview-window", "right:60%",
                        "--pointer", "▶",
                    ])
                    .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

        fzf.stdin.as_mut().unwrap().write_all(fzf_input.as_bytes())?;
        
        let output = fzf.wait_with_output()?;
        let selected_text = String::from_utf8_lossy(&output.stdout);

        if selected_text.trim().is_empty() {
                return Ok(Selection::Quit);
            }

        let parts: Vec<&str> = selected_text.split(" │ ").map(|s| s.trim()).collect();
        if parts.len() < 4 { return Ok(Selection::Quit); }
        
        let id_or_action = parts[3];

        if id_or_action == "next" {
                Ok(Selection::NextPage)
            } else if id_or_action == "prev" {
                Ok(Selection::PrevPage)
            } else if id_or_action == "quit" {
                Ok(Selection::Quit)
            } else {
                if let Some(video) = videos.iter().find(|v| v.id == id_or_action) {
                        Ok(Selection::Video(video.clone()))
                    } else {
                        Ok(Selection::Quit)
                    }
            }
}

pub fn select_format(formats: &[VideoFormat]) -> Result<Option<VideoFormat>> {
        let mut fzf_input = String::new();
        for f in formats {
                fzf_input.push_str(&format!("{:<8} │ {:<5} │ {:<10} │ {} │ {}\n", 
                            f.resolution, f.ext, format!("{}fps", f.fps), f.vcodec, f.id));
            }

        let mut fzf = Command::new("fzf")
                .args([
                        "--height", "40%", 
                        "--layout", "reverse", 
                        "--cycle", 
                        "--header", "Select Resolution", 
                        "--delimiter", " │ ",
                        "--with-nth", "1,2,3,4"
                    ])
                    .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

        fzf.stdin.as_mut().unwrap().write_all(fzf_input.as_bytes())?;
        let output = fzf.wait_with_output()?;
        let selected = String::from_utf8_lossy(&output.stdout);

        if selected.trim().is_empty() { return Ok(None); }

        let parts: Vec<&str> = selected.split(" │ ").map(|s| s.trim()).collect();
        if parts.len() < 5 { return Ok(None); }
        let selected_id = parts[4].trim();

        Ok(formats.iter().find(|f| f.id == selected_id).cloned())
}

pub fn select_action() -> Result<Option<Action>> {
        let mut fzf = Command::new("fzf")
                .args(["--height", "20%", "--layout", "reverse", "--cycle", "--header", "Action"])
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
