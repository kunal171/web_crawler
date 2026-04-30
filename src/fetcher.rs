use reqwest::StatusCode;

#[derive(Debug)]
pub struct FetchedPage {
    pub status: StatusCode,
    pub body: String,
}

pub async fn fetch_page(url: &str) -> Result<FetchedPage, reqwest::Error> {
    // Send a simple GET request. `.await` pauses this task while the network request runs.
    let response = reqwest::get(url).await?;

    // Keep the status before consuming the response body.
    let status = response.status();
    let body = response.text().await?;

    Ok(FetchedPage { status, body })
}
