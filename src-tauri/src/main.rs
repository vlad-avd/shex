extern crate core;

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenuItem};

use shex::{build_tray_menu, get_event_handler, load_config};

fn main() {
    let config = load_config().items;

    let tray_menu = build_tray_menu(&config)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(get_event_handler())
        .run(tauri::generate_context!())
        .expect("Unexpected error while launching shex");
}