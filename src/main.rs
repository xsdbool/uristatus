use std::io::{self, BufRead};
use reqwest::{Client, Response};

fn main() -> io::Result<()> {
    let client = Client::new();

    // Read from stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let url = line?;
        let resp: Response = client.get(&url).send()?;
        println!("{}: {}", url, resp.status());
    }

    Ok(())
}