//#![deny(warnings)]
#![warn(rust_2018_idioms)]
use std::env;

use hyper::{
    body::HttpBody as _,
    header::{AUTHORIZATION, USER_AGENT},
    Body, Client, Request,
};
use hyper_tls::HttpsConnector;
use tokio::io::{self, AsyncWriteExt as _};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();

    //if url.scheme_str() != Some("http") {
    //    println!("This example only works with 'http' URLs.");
    //    return Ok(());
    //}

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    //let client = Client::new();
    //let mut res = client.get(url).await?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let token = match env::var("TOKEN") {
        Ok(token) => token,
        Err(_) => {
            println!("no env var.");
            std::process::exit(1);
        }
    };

    let req = Request::builder()
        .uri(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(USER_AGENT, "Awesome-Octocat-App")
        .body(Body::from(""))
        .unwrap();

    let mut res = client.request(req).await?;
    //let mut res = client.get(url).await?;
    //assert_eq!(res.status(), 200);

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
}
