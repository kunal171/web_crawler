//! HTTP fetching: sends a GET request and returns the status code and response body.

use reqwest::StatusCode;

#[derive(Debug)]
pub struct FetchedPage {
    pub status: StatusCode,
    pub body: String,
}

/// Fetches a page over HTTP. `.await` yields control while the network request is in flight.
pub async fn fetch_page(url: &str) -> Result<FetchedPage, reqwest::Error> {
    let response = reqwest::get(url).await?;

    // Grab status before .text() consumes the response body.
    let status = response.status();
    let body = response.text().await?;

    Ok(FetchedPage { status, body })
}
