use std::collections::{HashSet, VecDeque};
use url::Url;
use std::error::Error;
use std::rc::Rc;
use crate::fetcher;
use crate::output;
use crate::parser;


// A simple web crawler that starts from a given URL and explores linked pages up to a specified depth.
struct CrawlEntry {
    url: Rc<Url>,
    depth: usize,
}   

// The main crawl function that manages the crawling process, including fetching pages, parsing them, and queuing new links.
pub async fn crawl(start_url: Url, max_pages: usize, max_depth: usize) -> Result<(), Box<dyn Error>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut pages_fetched: usize = 0;

    println!("Starting crawl from: {start_url}");
    println!("Max pages: {max_pages}, Max depth: {max_depth}");
    println!();

    queue.push_back(
        CrawlEntry {
            url: Rc::new(start_url),
            depth: 0,
        }
    );

    while let Some(entry) = queue.pop_front() {
        if pages_fetched >= max_pages {
            break;
        }

        if entry.depth >= max_depth {
            continue;
        }

        if !visited.insert(Rc::clone(&entry.url)) {
            continue; // Skip already visited URLs
        }

        println!("[{}/{}] Depth {} | Fetching: {}", pages_fetched + 1, max_pages, entry.depth, entry.url);

        let page = match fetcher::fetch_page(entry.url.as_str()).await {
            Ok(page) => page,
            Err(e) => {
                eprintln!("Failed to fetch {}: {}", entry.url, e);
                continue;
            }
        };

        if !page.status.is_success() {
            eprintln!("HTTP {} for {}", page.status, entry.url);
            continue;
        }

        let page_info = parser::parse_page_info(&entry.url, &page.body);
        output::print_page_summary(entry.url.as_str(), page.status, &page_info);
        println!();

        let mut new_count = 0;
        for link in page_info.links {
            let rc_link = Rc::new(link);
            if !visited.contains(&rc_link) {
                queue.push_back(CrawlEntry {
                    url: rc_link,
                    depth: entry.depth + 1,
                });
                new_count += 1;
            }
        }

        pages_fetched += 1;
        println!("  Queued {} new links | Queue size: {} | Visited: {}", new_count, queue.len(), visited.len());
        println!();

    }

    println!("Crawl complete: {pages_fetched} pages fetched");
    Ok(())
}