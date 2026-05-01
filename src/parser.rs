use scraper::{Html, Selector};

pub struct PageInfo {
    pub title: String,
    pub link_count: usize,
}

pub fn parse_page_info(body: &str) -> PageInfo {
    // Build an HTML document tree that can be queried with CSS selectors.
    let document = Html::parse_document(body);

    let title = extract_title(&document);
    let link_count = count_links(&document);

    PageInfo { title, link_count }
}

fn extract_title(document: &Html) -> String {
    let title_selector = Selector::parse("title").expect("Failed to parse title selector");

    document
        .select(&title_selector)
        .next()
        .map(|element| element.text().collect::<String>())
        .unwrap_or_else(|| "No title found".to_string())
}

fn count_links(document: &Html) -> usize {
    let link_selector = Selector::parse("a[href]").expect("Failed to parse link selector");

    document.select(&link_selector).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_title_and_link_count() {
        let html = r#"
            <html>
                <head><title>Example Page</title></head>
                <body>
                    <a href="/one">One</a>
                    <a href="https://example.com/two">Two</a>
                    <a>No href</a>
                </body>
            </html>
        "#;

        let page_info = parse_page_info(html);

        assert_eq!(page_info.title, "Example Page");
        assert_eq!(page_info.link_count, 2);
    }

    #[test]
    fn uses_fallback_title_when_missing() {
        let page_info = parse_page_info("<html><body><p>No title</p></body></html>");

        assert_eq!(page_info.title, "No title found");
        assert_eq!(page_info.link_count, 0);
    }
}
