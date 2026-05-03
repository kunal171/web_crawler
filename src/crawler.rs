//! Crawl loop: manages the BFS queue, visited set, and depth/page limits.
//! Coordinates fetcher, parser, and output to crawl same-domain pages.

use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::rc::Rc;
use url::Url;

use crate::fetcher;
use crate::output;
use crate::parser;

/// Each entry in the crawl queue pairs a URL with how many hops it is from the start URL.
struct CrawlEntry {
    url: Rc<Url>,
    depth: usize,
}

/// Breadth-first crawl starting from `start_url`.
///
/// Stops when `max_pages` have been successfully fetched or no queued URL
/// is within `max_depth` hops of the start.
pub async fn crawl(start_url: Url, max_pages: usize, max_depth: usize) -> Result<(), Box<dyn Error>> {
    // VecDeque gives FIFO order, so shallower pages are visited before deeper ones.
    let mut queue: VecDeque<CrawlEntry> = VecDeque::new();
    // Rc<Url> lets the visited set and queue share URL ownership without deep-cloning the string.
    let mut visited: HashSet<Rc<Url>> = HashSet::new();
    let mut pages_fetched: usize = 0;

    println!("Starting crawl from: {start_url}");
    println!("Max pages: {max_pages}, Max depth: {max_depth}");
    println!();

    queue.push_back(CrawlEntry {
        url: Rc::new(start_url),
        depth: 0,
    });

    while let Some(entry) = queue.pop_front() {
        if pages_fetched >= max_pages {
            break;
        }

        if entry.depth >= max_depth {
            continue;
        }

        // insert() returns false if the URL was already present — acts as both check and mark.
        if !visited.insert(Rc::clone(&entry.url)) {
            continue;
        }

        println!(
            "[{}/{}] Depth {} | Fetching: {}",
            pages_fetched + 1,
            max_pages,
            entry.depth,
            entry.url
        );

        // Non-fatal: a single failed fetch shouldn't stop the entire crawl.
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

        // Only enqueue links not already visited to keep the queue small.
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
        println!(
            "  Queued {} new links | Queue size: {} | Visited: {}",
            new_count,
            queue.len(),
            visited.len()
        );
        println!();
    }

    println!("Crawl complete: {pages_fetched} pages fetched");
    Ok(())
}
