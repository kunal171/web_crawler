//! HTML parsing: extracts the page title and same-domain links from a fetched page body.

use scraper::{Html, Selector};
use url::Url;

pub struct PageInfo {
    pub title: String,
    pub links: Vec<Url>,
    pub link_count: usize,
}

/// Parses an HTML body and extracts the page title and same-domain HTTP/HTTPS links.
pub fn parse_page_info(base_url: &Url, body: &str) -> PageInfo {
    let document = Html::parse_document(body);

    let title = extract_title(&document);
    let links = extract_links(&document, base_url);
    let link_count = links.len();

    PageInfo {
        title,
        links,
        link_count,
    }
}

fn extract_title(document: &Html) -> String {
    let title_selector = Selector::parse("title").expect("Failed to parse title selector");

    document
        .select(&title_selector)
        .next()
        .map(|element| element.text().collect::<String>())
        .unwrap_or_else(|| "No title found".to_string())
}

/// Resolves each href against `base_url` so relative paths become absolute URLs,
/// then keeps only HTTP/HTTPS links on the same domain.
fn extract_links(document: &Html, base_url: &Url) -> Vec<Url> {
    let link_selector = Selector::parse("a[href]").expect("Failed to parse link selector");

    document
        .select(&link_selector)
        .filter_map(|element| element.value().attr("href"))
        // join() resolves relative hrefs ("/about", "../page") against the base URL.
        .filter_map(|href| base_url.join(href).ok())
        .filter(|url| url.scheme() == "http" || url.scheme() == "https")
        // Same-domain only: prevents the crawler from escaping to external sites.
        .filter(|url| url.host() == base_url.host())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_title_and_normalizes_http_links() {
        let html = r#"
            <html>
                <head><title>Example Page</title></head>
                <body>
                    <a href="/one">One</a>
                    <a href="https://example.com/two">Two</a>
                    <a href="../three">Three</a>
                    <a href="mailto:test@example.com">Email</a>
                    <a>No href</a>
                </body>
            </html>
        "#;

        let base_url = Url::parse("https://example.com/docs/index.html").unwrap();
        let page_info = parse_page_info(&base_url, html);

        assert_eq!(page_info.title, "Example Page");
        assert_eq!(page_info.link_count, 3);
        assert_eq!(page_info.links.len(), 3);
        assert_eq!(page_info.links[0].as_str(), "https://example.com/one");
        assert_eq!(page_info.links[1].as_str(), "https://example.com/two");
        assert_eq!(page_info.links[2].as_str(), "https://example.com/three");
    }

    #[test]
    fn uses_fallback_title_when_missing() {
        let base_url = Url::parse("https://example.com").unwrap();
        let page_info = parse_page_info(&base_url, "<html><body><p>No title</p></body></html>");

        assert_eq!(page_info.title, "No title found");
        assert_eq!(page_info.link_count, 0);
    }
}
