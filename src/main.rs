use clap::Parser;
use reqwest::Url;
use wikimap::path::PathTracer;

#[derive(Parser)]
#[command(name = "wikimap")]
#[command(about = "Blazingly fast mapper between wikipedia articles")]
pub struct Config {
    #[arg(help = "First article's URL")]
    pub url_1: Url,

    #[arg(help = "Second article's URL")]
    pub url_2: Url,

    #[arg(
        short = 'l',
        long,
        default_value_t = 15,
        help = "Path's limit size, will error if cannot find a shorter path"
    )]
    pub limit: u32,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = Config::parse();

    let mut path_tracer = PathTracer::new(config.url_1, config.url_2);
    let path = path_tracer.trace_path(config.limit).await?;

    println!("Found path in {} steps\n", path.len());

    let mut iterator = path.iter().peekable();

    while let Some(link) = iterator.next() {
        println!("● {}", link);
        if iterator.peek().is_some() {
            println!("||");
        }
    }

    Ok(())
}
