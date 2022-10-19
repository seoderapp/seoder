#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate seoder_web;

use seoder_web::tokio;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|_| {
            tauri::async_runtime::spawn(async move { seoder_web::start().await.unwrap() });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
