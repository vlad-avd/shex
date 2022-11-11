extern crate core;

use tauri::{AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu, Wry};

use crate::config::{find_item, load_config, ShexMenuItem};

mod config;

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

fn handle_tray_event(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => app.exit(0),
                item => {
                    // TODO: find executable in config
                    // TODO: run executable
                    println!("{:?}", find_item(&load_config().items, &id));
                }
            }
        }
        _ => {}
    }
}

fn build_tray_menu(items: &Vec<Box<ShexMenuItem>>) -> SystemTrayMenu {
    let mut menu = SystemTrayMenu::new();

    for item in items.iter() {
        let title = &item.title;

        if item.has_submenu() {
            menu = menu.add_submenu(SystemTraySubmenu::new(title, build_tray_menu(&item.child_items)));
        } else {
            menu = menu.add_item(CustomMenuItem::new(title, title));
        }
    }

    menu
}