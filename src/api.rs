use tokio::fs::File;
use tokio::io;

pub async fn download(link: &str, name: &str) {
    let resp = reqwest::get(link).await.expect("request failed");
    let body = resp.text().await.expect("Can't convert body to bytes!");
    let mut out = File::create(name).await.expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).await.expect("failed to copy content");
}