mod cli;
mod crawler;
mod fetcher;
mod output;
mod parser;

/// Tokio's #[tokio::main] sets up the async runtime so .await works inside main.
#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let url = cli::read_start_url()?;
    let parsed_url = url::Url::parse(&url)?;

    crawler::crawl(parsed_url, 10, 2).await?;

    Ok(())
}
