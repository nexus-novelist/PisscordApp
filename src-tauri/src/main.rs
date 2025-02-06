// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, generate_context};

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        let webview = app.get_webview_window("main").unwrap();
        webview.eval("window.location.replace('https://pisscord.up.railway.app')").unwrap();
        Ok(())
      })
      .run(tauri::generate_context!()) //cannot find proc-macro server in sysroot `/usr`
      .expect("error running tauri app");
}