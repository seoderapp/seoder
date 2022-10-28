#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate seoder_web;

use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use seoder_web::tokio;
use tauri::Manager;
use tauri::Window;

#[cfg(target_os = "macos")]
fn set_transparent_titlebar(win: Window, transparent: bool) {
    use cocoa::appkit::NSWindowTitleVisibility;

    unsafe {
        let id = win.ns_window().unwrap() as cocoa::base::id;

        let mut style_mask = id.styleMask();

        style_mask.set(
            NSWindowStyleMask::NSFullSizeContentViewWindowMask,
            transparent,
        );

        id.setStyleMask_(style_mask);

        id.setTitleVisibility_(if transparent {
            NSWindowTitleVisibility::NSWindowTitleHidden
        } else {
            NSWindowTitleVisibility::NSWindowTitleVisible
        });

        id.setTitlebarAppearsTransparent_(if transparent {
            cocoa::base::YES
        } else {
            cocoa::base::NO
        });
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::spawn(async move { seoder_web::start().await.unwrap() });
            let win = app.get_window("main").unwrap();
            set_transparent_titlebar(win, true);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
