use std::path::Path;
use reqwest::Client;
use super::launch;
use super::assets;
use super::download;

pub async fn bootstrap() {
    match Path::new(assets::constants::SERVER_FILE).exists() {
        true => println!("Server file already exists!"),
        false => {
            println!("Server file doesn't exist...");
            println!("Downloading the server file!");
            // api::download_v1(assets::constants::DOWNLOAD, "server.jar").await
            download::download_v2(&Client::new(), assets::constants::DOWNLOAD, "server.jar").await.expect("Failed to download file");
        }
    }

    launch::start().await;
}