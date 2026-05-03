mod cli;
mod fetcher;
mod output;
mod parser;

// Tokio starts the async runtime so this async main function can use `.await`.
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
    let page = fetcher::fetch_page(parsed_url.as_str()).await?;

    if !page.status.is_success() {
        eprintln!("Failed to fetch the URL: HTTP {}", page.status);
        return Ok(());
    }

    let page_info = parser::parse_page_info(&parsed_url, &page.body);
    output::print_page_summary(&url, page.status, &page_info);

    Ok(())
}
