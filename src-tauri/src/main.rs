extern crate core;

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenuItem};

use shex::{build_tray_menu, handle_tray_event, load_config};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let click = CustomMenuItem::new("click".to_string(), "Click here");

    let config_items = &load_config().items;

    let tray_menu = build_tray_menu(config_items)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(click)
        .add_item(quit);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(handle_tray_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}