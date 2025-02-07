// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::{Manager, WebviewWindow};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

#[tauri::command]
fn toggle_mute(window: WebviewWindow) {
    window
        .eval("window.dispatchEvent(new Event('toggle-mute'))")
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                let webview = app.get_webview_window("main").unwrap();
                use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};
                let toggle_mute_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyX);

                let webview = Arc::new(webview);
                let webview_ref = Arc::clone(&webview);
                let _ = app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            println!("{:?}", shortcut);
                            if shortcut == &toggle_mute_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        println!("Toggle mute");
                                        toggle_mute(webview_ref.as_ref().clone());
                                    }
                                    ShortcutState::Released => {}
                                }
                            }
                        })
                        .build(),
                );

                let _ = app.global_shortcut().register(toggle_mute_shortcut);

                webview
                    .eval("window.location.replace('https://pisscord.up.railway.app')")
                    .unwrap();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
