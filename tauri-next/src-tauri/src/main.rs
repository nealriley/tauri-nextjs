// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, Builder, WindowBuilder};
use std::process::Command;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let mut echo_hello = Command::new("sh");
    echo_hello.arg("-c").arg(name);
    let hello_1 = echo_hello.output().expect("failed to execute process");
    let stdout = String::from_utf8_lossy(&hello_1.stdout);
    println!("{}", stdout);
    format!("{}", stdout)   
}

fn main() {
    let fullscreen = CustomMenuItem::new("fullscreen".to_string(), "Enter Fullscreen");
    let standard = CustomMenuItem::new("standard".to_string(), "Leave Fullscreen");
    let menu = Menu::new().add_item(fullscreen).add_item(standard);
    
    Builder::default()
        .menu(menu)
        .invoke_handler(tauri::generate_handler![greet])
        .on_menu_event(|event| {
            if event.menu_item_id() == "fullscreen" {
                // Toggle fullscreen here
                event.window().set_fullscreen(true);
            } else if (event.menu_item_id() == "standard") {
                event.window().set_fullscreen(false);
            }
        })
        
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


