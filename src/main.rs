use std::io::{self, BufRead};
use reqwest::{Client, Response, Error as ReqwestError};
use tokio;

#[tokio::main]
async fn main() -> io::Result<()> {
    let client = Client::new();

    // Read from stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let url = line?;
        request_url(client.clone(), url).await?;
    }

    Ok(())
}

async fn request_url(client: Client, url: String) -> io::Result<()> {
    let resp: Result<Response, ReqwestError> = client.get(&url).send().await;
    match resp {
        Ok(response) => println!("{}: {}", url, response.status()),
        Err(err) => println!("{}: {}", url, err),
    }

    Ok(())
}
