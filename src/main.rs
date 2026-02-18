mod runner;
mod types;
mod ui;
mod youtube;

use anyhow::Result;
use clap::Parser;
use console::style;

#[derive(Parser)]
struct Args {
        #[arg(short, long)]
        sub: bool,
        query: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
        ui::show_header()?;

        let args = Args::parse();
        let mut search_query = args.query.join(" ");

        if search_query.is_empty() {
                search_query = ui::get_user_input("Search:")?;
            }
        if search_query.is_empty() { return Ok(()); }

        println!("\n{}", style("Fetching...").yellow().dim());
        let videos = youtube::search(&search_query)?;
        
        if videos.is_empty() {
                println!("{}", style("No videos found.").red());
                return Ok(());
            }

        let selected_video = match ui::select_video(&videos)? {
                Some(v) => v,
                None => return Ok(()),
            };
        
        let video_url = format!("https://www.youtube.com/watch?v={}", selected_video.id);

        println!("\n{}", style("Fetching formats...").yellow().dim());
        let formats = youtube::get_formats(&video_url)?;

        if formats.is_empty() {
                println!("{}", style("No formats found.").red());
                return Ok(());
            }

        let selected_format = match ui::select_format(&formats)? {
                Some(f) => f,
                None => return Ok(()),
            };

        if let Some(action) = ui::select_action()? {
                match action {
                        types::Action::Stream => {
                                runner::stream(&video_url, &selected_format.id)?;
                            }
                        types::Action::Download => {
                                runner::download(&video_url, &selected_format.id, args.sub)?;
                            }
                    }
            }

        Ok(())
}
