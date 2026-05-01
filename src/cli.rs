use std::env;

pub fn read_start_url() -> Result<String, String> {
    env::args()
        .nth(1)
        .ok_or_else(|| "Usage: cargo run -- <url>".to_string())
}
