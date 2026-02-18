use anyhow::Result;
use console::style;
use std::process::Command;

pub fn stream(url: &str, fmt: &str) -> Result<()> {
        println!("{}", style("Starting MPV...").cyan());
        let combined = format!("{}+bestaudio/best", fmt);
        Command::new("mpv")
            .args([format!("--ytdl-format={}", combined), url.to_string()])
                .status()?;
        Ok(())
}

pub fn download(url: &str, fmt: &str, subs: bool) -> Result<()> {
        println!("{}", style("Starting Download...").green());
        let combined = format!("{}+bestaudio/best", fmt);
        let mut cmd = Command::new("yt-dlp");
        cmd.args(["-f", &combined, "--no-mtime", "--progress", url]);
        
        if subs {
                cmd.args(["--write-auto-subs", "--sub-lang", "en,id", "--embed-subs"]);
            }
        
        cmd.status()?;
        Ok(())
}
