use crate::parser::PageInfo;
use reqwest::StatusCode;

pub fn print_page_summary(url: &str, status: StatusCode, page_info: &PageInfo) {
    println!("URL: {url}");
    println!("Status: {status}");
    println!("Title: {}", page_info.title);
    println!("Links found: {}", page_info.link_count);
    for link in &page_info.links {
        println!("- {link}");
    }
}
