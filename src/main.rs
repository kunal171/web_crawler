use scraper::{Html, Selector};
use std::env;

#[tokio::main]
async fn main() {
    // Get the URL from command line arguments
    let url = env::args().nth(1).expect("Usage: cargo run -- <url>");

    let response = reqwest::get(&url).await.expect("Failed to fetch the URL");

    let status = response.status();
    if !status.is_success() {
        eprintln!("Failed to fetch the URL: HTTP {}", status);
        return;
    }

    let body = response.text().await.expect("Failed to read the response body");

    let document = Html::parse_document(&body);

    let title_selector = Selector::parse("title").expect("Failed to parse title selector");

    let title = document.
        select(&title_selector)
        .next()
        .map(|element| element.text().collect::<String>())
        .unwrap_or_else(|| "No title found".to_string());

    let link_selector = Selector::parse("a[href]")
        .expect("Failed to parse link selector");

    let link_count = document.select(&link_selector).count();

    println!("URL: {url}");
    println!("Status: {status}");
    println!("Title: {title}");
    println!("Links found: {link_count}");
}
