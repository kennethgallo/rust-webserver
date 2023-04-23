use reqwest::{Error as ReqwestError, StatusCode};
use std::io;
use tokio;

#[derive(Debug)]
enum FetchError {
    Reqwest(ReqwestError),
    HttpStatus(StatusCode),
}

impl From<ReqwestError> for FetchError {
    fn from(err: ReqwestError) -> Self {
        Self::Reqwest(err)
    }
}

async fn fetch_web_page(url: &str) -> Result<String, FetchError> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let content = response.text().await?;
        Ok(content)
    } else {
        Err(FetchError::HttpStatus(response.status()))
    }
}

#[tokio::main]
async fn main() {
    println!("Please enter the URL:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let url = input.trim();

    match fetch_web_page(url).await {
        Ok(content) => println!("Web page content:\n{}", content),
        Err(e) => eprintln!("Failed to get the web page. Error: {:?}", e),
    }
}