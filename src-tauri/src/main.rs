extern crate core;

use tauri::{AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, Wry};

use crate::config::load_config;

mod config;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let click = CustomMenuItem::new("click".to_string(), "Click here");

    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_item(click);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(handle_tray_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_tray_event(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => app.exit(0),
                item => {
                    println!("Clicked on: {item}");
                    println!("{:?}", load_config());
                }
            }
        }
        _ => {}
    }
}
