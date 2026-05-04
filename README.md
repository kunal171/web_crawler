# Web Crawler

An async web crawler in Rust that crawls same-domain pages with bounded concurrency.

## What It Does

Accepts a starting URL, fetches pages breadth-first, extracts titles and links, and follows same-domain links up to configurable depth and page limits — fetching multiple pages concurrently.

```text
start URL
    |
fetch pages concurrently (bounded by semaphore)
    |
extract title and same-domain links
    |
queue unseen links, track visited with HashSet
    |
stop at max pages or max depth
```

## Project Structure

```text
web_crawler/
├── Cargo.toml
└── src/
    ├── main.rs       — entry point, calls crawler::crawl()
    ├── cli.rs        — reads start URL from args
    ├── fetcher.rs    — async HTTP GET with reqwest
    ├── parser.rs     — HTML parsing, title/link extraction, same-domain filtering
    ├── output.rs     — prints page summary to stdout
    └── crawler.rs    — BFS crawl loop, concurrency control, queue, visited set
```

## Architecture

```text
cli::read_start_url()
    |
url::Url::parse()
    |
crawler::crawl(start_url, max_pages, max_depth, max_concurrency)
    |
    loop:
        Phase 1: pop from VecDeque, check visited HashSet, spawn fetch tasks
                 (bounded by tokio::sync::Semaphore)
        Phase 2: await completed tasks from JoinSet, process results,
                 enqueue new same-domain links
    |
    each spawned task:
        fetcher::fetch_page()  ->  parser::parse_page_info()  ->  CrawlResult
```

Key design decisions:

- `Arc<Url>` for shared ownership across spawned tasks (`Rc` is not `Send`)
- `Semaphore` with `acquire_owned()` to bound in-flight requests
- `JoinSet` to manage dynamic spawned tasks and collect results as they complete
- `HashSet<Arc<Url>>` for O(1) visited deduplication
- `VecDeque<CrawlEntry>` for BFS ordering (shallower pages first)
- Same-domain filtering in parser prevents escaping to external sites
- Non-fatal error handling: failed fetches log and continue

## Usage

```bash
# Crawl with default limits (10 pages, depth 2, concurrency 4)
cargo run -- https://example.com
```

## Dependencies

- `tokio` — async runtime
- `reqwest` — HTTP client
- `scraper` — HTML parsing with CSS selectors
- `url` — URL parsing and resolution
- `log` — logging

## Commands

```bash
cargo run -- <url>
cargo test
cargo check
cargo fmt --check
cargo clippy
```

## Tests

Unit tests in `parser.rs`:

- Title extraction and HTTP link normalization (relative, absolute, cross-scheme)
- Same-domain filtering (drops external links, `mailto:`, etc.)
- Missing title fallback
