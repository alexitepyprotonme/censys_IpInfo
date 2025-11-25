use std::fs::File;
use std::io::{self, Read, Write};
use flate2::read::GzDecoder;
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"));
    headers.insert(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_static(" cf_clearance=BvcjVIBxjQmIHMjpTnwyELAC9GlR_7ike7UjZUKNVXI-1708519366-1.0-ATXDYeH+xyDjIYzJJYv5vUxc7deo3ICpi0Fs97I+Rdtihpz7y7Z8EBWc6YEWmnKyItFW+mSusEvXVcQgeCtV4w0=; __cf_bm=hH3rQk6fx_zYeMYjctbKB4JUOcUY4EP6KvnzWgQX_sE-1708519380-1.0-AQ1Eu6WTr81qSYvnM7YWzVNFHrO/pMqRCdQzihoKslTTqHXKWE9yCAc+XARMJMbY9rzZF9NUBw++/3kkczdn2cE=; search.censys.io.session.id=sfj54nnUoUTFK7Th4nyoxJsT-c9ZdT1GgjNBJOJeaJxEMvOMeY3KVI0csy8m48_cnRzUR5Q-zbQ87GXUoxa1Q6vw3CXC6hlPtd5XxEGoE12Hz6vL6NpUhTPGEeh8SLHDpqxP95k0AK7qby86azmhreksV1JHfvBBh3dxGA"));
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0\n\n"));

    let response = client
        .get("https://search.censys.io/hosts/8.8.8.8/data/json")
        .headers(headers)
        .send()?;

    let mut decoder = GzDecoder::new(response);
    let mut response_text = String::new();
    decoder.read_to_string(&mut response_text)?;

    let mut file = File::create("response.txt")?;
    file.write_all(response_text.as_bytes())?;

    Ok(())
}
