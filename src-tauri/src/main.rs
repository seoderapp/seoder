#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate seoder_web;

use seoder_web::tokio;

#[tokio::main]
async fn main() {
    tokio::spawn(async move { seoder_web::start().await.unwrap() });

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
