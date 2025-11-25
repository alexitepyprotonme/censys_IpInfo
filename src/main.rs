use reqwest::blocking::Client;
use std::io::Read; // Import Read trait for reading response body // Import reqwest blocking client

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a reqwest client
    let client = Client::new();

    // Send the request
    let mut response = client.get("https://search.censys.io/hosts/8.8.8.8/data/json")
        // .header("Host", "search.censys.io")
        .header("Cookie", "cf_clearance=DTNcMOzInEWGXkPr7ABrB8pgvJQF7r2NBNOMrAaMecs-1708589476-1.0-AUL7qg+wNCsZ+Jswuqh1zLJhta1uNxOR+ie5TW4z9jFZGh+FyMT4o/M6KqVOwZDXkF4pWBjFdajaulO2O7Q1vLY=; __cf_bm=wwIMTiiUo8_Tx4ckAi6G18AmdZ5huw5UtwzCDj335tM-1708589481-1.0-AXEk/YHMYs0Jz+pAuuNXtRUgQQ97q253TaBSuepnUMSBwiaNYXZoK0zGs3upEBSV3Lj4VQiNyEhaDiiXA+I/qOI=; search.censys.io.session.id=CBr9Mmow4i3bCzhvbPEEFvyXqc6aF8ZD0kN484CkVjtCvKOyp6bteb6aGN0WfKRSVWdth4NbL1w2gX59jdfBIy9WCcCrOr_yCQNSGX9jIxQqWzhRhWuBbQp6HroYBxc5EcoNTSfwc2apeiVbLx_2ZaUdhWIbHFRWlRJ-cQ")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0")
        // .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
        // .header("Accept-Language", "en-US,en;q=0.5")
        // .header("Accept-Encoding", "gzip, deflate, br")
        // .header("Referer", "https://search.censys.io/hosts/8.8.8.8/data/json")
        // .header("Upgrade-Insecure-Requests", "1")
        // .header("Sec-Fetch-Dest", "document")
        // .header("Sec-Fetch-Mode", "navigate")
        // .header("Sec-Fetch-Site", "same-origin")
        // .header("Sec-Fetch-User", "?1")
        // .header("Te", "trailers")
        .header("Connection", "keep-alive\r\n\r\n")
        .send()?; // Send the request and unwrap the response

    // Check if the request was successful
    if response.status().is_success() {
        // Read the response body to a string
        let mut body = String::new();
        response.read_to_string(&mut body)?;

        // Print the response body
        println!("{}", body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}
