# Web Crawler

A Rust learning project for practicing async Rust, HTTP requests, HTML parsing,
URL handling, and controlled concurrency.

This is the Phase 1C project in the Rust foundations track.

## What Is A Web Crawler

A web crawler is a program that automatically visits web pages, reads their
content, extracts links from them, and then follows those links to discover more
pages.

In simple terms:

```text
start with one URL
    |
download the page
    |
read the HTML
    |
find links on the page
    |
visit new links
    |
repeat with limits
```

Search engines use crawlers to discover pages on the internet. Monitoring tools
use crawlers to check whether pages are available. Data tools use crawlers to
collect structured information from websites. Security tools use crawlers to map
an application and find reachable pages.

## What This Crawler Will Do

This project will start small and grow step by step.

The crawler will eventually:

- accept a starting URL
- fetch the HTML page for that URL
- read the HTTP status code
- extract the page title
- find links inside `<a href="...">` tags
- convert relative links into absolute URLs
- avoid visiting the same URL repeatedly
- crawl pages up to a maximum depth or page limit
- fetch multiple pages concurrently with a safe limit

## Why It Is Useful

A web crawler is useful because it teaches how real networked systems move
through external data.

For learning Rust, it is especially useful because it combines:

- async programming
- HTTP requests
- HTML parsing
- URL normalization
- queues
- duplicate tracking
- error handling
- concurrency limits

It is also a good bridge toward distributed systems because a crawler has to
manage external services, failures, rate limits, retries, and resource usage.

## Goal

Build a simple web crawler step by step:

```text
start URL
    |
fetch HTML page
    |
extract page title and links
    |
normalize links
    |
avoid visiting duplicates
    |
crawl with depth/page limits
    |
add controlled concurrency
```

The purpose is not to build a production crawler immediately. The purpose is to
learn the async and networking pieces carefully.

## Current State

Milestone 1 is complete. Milestone 2 (link normalization) is mostly complete.

Current files:

```text
web_crawler/
├── Cargo.toml
├── README.md
├── .gitignore
└── src/
    ├── main.rs
    ├── cli.rs
    ├── fetcher.rs
    ├── parser.rs
    └── output.rs
```

Current progress:

- Dependencies for Tokio, Reqwest, Scraper, URL parsing, and logging have been added.
- The implementation is split into modules: `cli`, `fetcher`, `parser`, and `output`.
- `cli.rs` reads the starting URL from command-line args.
- `fetcher.rs` fetches one page with `reqwest` and returns status and body.
- `parser.rs` parses HTML with `scraper`, extracts the page title, normalizes relative links with `url::Url::join`, and filters for HTTP/HTTPS links.
- `output.rs` prints the URL, status, title, link count, and each link.
- `parser.rs` has unit tests for title extraction, link normalization, unsupported link filtering, and missing-title fallback.
- `cargo test` passes with 2 parser tests.
- `cargo fmt --check` passes.

Current command:

```text
cargo run -- https://example.com
    |
fetch page
    |
print status, title, link count, and normalized links
```

## Planned Learning Concepts

- async Rust basics
- `async fn`
- `.await`
- Tokio runtime
- HTTP requests
- response status handling
- HTML parsing
- CSS selectors
- URL parsing and normalization
- queues with `VecDeque`
- duplicate tracking with `HashSet`
- crawl depth limits
- page count limits
- controlled concurrency
- error handling with `Result`

## Suggested Dependencies

These are likely dependencies for the first useful version:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["rustls-tls"] }
scraper = "0.22"
url = "2"
```

What each one is for:

- `tokio` runs async Rust code.
- `reqwest` makes HTTP requests.
- `scraper` parses HTML and extracts elements.
- `url` parses and joins URLs safely.

## Milestone 1

Status: complete.

Fetch one URL and print:

- HTTP status
- page title
- number of links found
- each normalized link

Target flow:

```text
cargo run -- https://example.com
    |
fetch page
    |
parse HTML
    |
print title and link count
```

Why this comes first:

```text
If the crawler cannot fetch and parse one page,
it should not start crawling multiple pages yet.
```

## Milestone 2

Status: mostly complete.

Extract and normalize links from the page.

Completed:

- Only looks at `<a href="...">`.
- Converts relative links into absolute URLs using `Url::join`.
- Filters for HTTP/HTTPS links only (ignores `mailto:`, `javascript:`, etc.).
- Unit tests cover normalization, filtering, and missing-title fallback.

Remaining:

- Same-domain filtering (only follow links on the starting domain).

## Milestone 3

Add a crawl queue:

```text
VecDeque<Url> queue
HashSet<Url> visited
```

Basic loop:

```text
pop URL from queue
skip if already visited
fetch page
extract links
push new links into queue
stop at max pages or max depth
```

## Milestone 4

Add controlled concurrency.

The crawler should fetch multiple pages at once, but not unlimited pages.

Important lesson:

```text
async concurrency needs limits
```

Without limits, a crawler can overload your machine or the target site.

## Useful Commands

From this directory:

```bash
cargo run
cargo check
cargo test
cargo fmt --check
cargo clippy
```

## Learning Rule

For this project, build small milestones and benchmark/understand each one before
adding more concurrency.
