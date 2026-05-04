//! Crawl loop: manages the BFS queue, visited set, depth/page limits, and concurrent fetching.
//! Uses Arc instead of Rc so URLs can be shared across tokio::spawn task boundaries (Send required).

use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::sync::Arc;

use reqwest::StatusCode;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use url::Url;

use crate::fetcher;
use crate::output;
use crate::parser;
use crate::parser::PageInfo;

/// Each entry in the crawl queue pairs a URL with how many hops it is from the start URL.
struct CrawlEntry {
    url: Arc<Url>,
    depth: usize,
}

/// Result sent back from a spawned fetch task to the main crawl loop.
struct CrawlResult {
    url: Arc<Url>,
    depth: usize,
    status: StatusCode,
    page_info: PageInfo,
}

/// Breadth-first crawl starting from `start_url` with bounded concurrency.
///
/// Spawns up to `max_concurrency` fetch tasks at once using a semaphore.
/// Stops when `max_pages` have been successfully fetched or no queued URL
/// is within `max_depth` hops of the start.
pub async fn crawl(
    start_url: Url,
    max_pages: usize,
    max_depth: usize,
    max_concurrency: usize,
) -> Result<(), Box<dyn Error>> {
    let mut queue: VecDeque<CrawlEntry> = VecDeque::new();
    // Arc<Url> lets the visited set, queue, and spawned tasks share URL ownership cheaply.
    let mut visited: HashSet<Arc<Url>> = HashSet::new();
    let mut pages_fetched: usize = 0;

    // Semaphore limits how many fetches run at the same time.
    // acquire_owned() blocks when all permits are taken; the permit drops when the task finishes.
    let semaphore = Arc::new(Semaphore::new(max_concurrency));

    // JoinSet manages a dynamic set of spawned tasks and lets us await them as they complete.
    let mut tasks: JoinSet<Option<CrawlResult>> = JoinSet::new();

    println!("Starting crawl from: {start_url}");
    println!("Max pages: {max_pages}, Max depth: {max_depth}, Concurrency: {max_concurrency}");
    println!();

    queue.push_back(CrawlEntry {
        url: Arc::new(start_url),
        depth: 0,
    });

    loop {
        // Phase 1: spawn fetch tasks for queued URLs up to the concurrency limit.
        while let Some(entry) = queue.pop_front() {
            if pages_fetched + tasks.len() >= max_pages {
                break;
            }

            if entry.depth >= max_depth {
                continue;
            }

            if !visited.insert(Arc::clone(&entry.url)) {
                continue;
            }

            // Wait for a semaphore permit before spawning — this is what bounds concurrency.
            // acquire_owned() moves the permit into the task so it drops when the task ends.
            let permit = Arc::clone(&semaphore).acquire_owned().await?;
            let url = Arc::clone(&entry.url);
            let depth = entry.depth;

            tasks.spawn(async move {
                let page = match fetcher::fetch_page(url.as_str()).await {
                    Ok(page) => page,
                    Err(e) => {
                        eprintln!("Failed to fetch {}: {e}", url);
                        drop(permit);
                        return None;
                    }
                };

                if !page.status.is_success() {
                    eprintln!("HTTP {} for {}", page.status, url);
                    drop(permit);
                    return None;
                }

                let page_info = parser::parse_page_info(&url, &page.body);
                let status = page.status;

                // Permit drops here, freeing a concurrency slot for the next task.
                drop(permit);

                Some(CrawlResult {
                    url,
                    depth,
                    status,
                    page_info,
                })
            });
        }

        // No tasks in flight and queue is empty — crawl is done.
        if tasks.is_empty() {
            break;
        }

        // Phase 2: wait for the next task to finish and process its results.
        if let Some(join_result) = tasks.join_next().await {
            let result = join_result?;

            if let Some(crawl_result) = result {
                output::print_page_summary(
                    crawl_result.url.as_str(),
                    crawl_result.status,
                    &crawl_result.page_info,
                );
                println!();

                pages_fetched += 1;

                // Enqueue discovered links that haven't been visited yet.
                let mut new_count = 0;
                for link in crawl_result.page_info.links {
                    let arc_link = Arc::new(link);
                    if !visited.contains(&arc_link) {
                        queue.push_back(CrawlEntry {
                            url: arc_link,
                            depth: crawl_result.depth + 1,
                        });
                        new_count += 1;
                    }
                }

                println!(
                    "  [{pages_fetched}/{max_pages}] Queued {new_count} new links | Queue: {} | Visited: {}",
                    queue.len(),
                    visited.len()
                );
                println!();

                if pages_fetched >= max_pages {
                    break;
                }
            }
        }
    }

    println!("Crawl complete: {pages_fetched} pages fetched");
    Ok(())
}
