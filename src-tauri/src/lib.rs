use std::{fs, process};
use std::env::var;
use std::fs::File;
use std::path::Path;

use tauri::{AppHandle, CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTraySubmenu, Wry};

use crate::config::{Config, Executable, ShexMenuItem};

mod config;

pub fn load_config() -> Config {
    let configs_dir = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home| format!("{home}/.config")))
        .unwrap();

    let config_path = format!("{configs_dir}/shex");
    fs::create_dir_all(&config_path).unwrap();

    let path = format!("{config_path}/scripts_config.json");
    let config = Path::new(&path);
    if !config.exists() {
        match File::create(config) {
            Ok(file) => { serde_json::to_writer_pretty(&file, &build_demo_config()).expect("TODO: panic message");},
            Err(_) => println!("Oops")
        }
    }

    let config = match fs::read_to_string(&config) {
        Ok(data) => data,
        //TODO: create a file and fill
        Err(_) => String::new(),
    };

    match serde_json::from_str::<Config>(&config) {
        Ok(config) => config,
        Err(_) => Config::new(),
    }
}

fn build_demo_config() -> Config {
    Config {
        items: vec![
            Box::new(
                ShexMenuItem {
                    title: "echo".to_string(),
                    daemon: false,
                    executable: Executable::Command {
                        body: String::from("echo $HOME")
                    },
                    child_items: vec![]
                },
            )
        ]
    }
}

// now id is the title field
pub fn find_item<'a>(items: &'a Vec<Box<ShexMenuItem>>, id: & String) -> Option<&'a Box<ShexMenuItem>> {
    let mut result = None;
    for item in items {
        if item.has_submenu() {
            result = find_item(&item.child_items, id);
        } else {
            if id.eq(&item.title) {
                let a = item;
                return Some(a);
            }
        }
    }
    result
}

pub fn handle_tray_event(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => app.exit(0),
                _item => {
                    // TODO: store config to context
                    let config = &load_config().items;
                    // TODO: find executable in config
                    // TODO: run executable
                    if let Some(item) = find_item(config, &id) {
                        handle_item_click(item)
                    }
                    // println!("{:?}", find_item(&load_config().items, &id));
                }
            }
        }
        _ => {}
    }
}

pub fn build_tray_menu(items: &Vec<Box<ShexMenuItem>>) -> SystemTrayMenu {
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

pub fn handle_item_click(item: &Box<ShexMenuItem>) {
    match item.executable {
        Executable::Command { ref body } => {
            run_executable(&item, &body);
        }
        Executable::Script { ref path } => {
            run_executable(&item, &path);
        }
    }
}

fn run_executable(item: &ShexMenuItem, str: &str) {
    match item.is_daemon() {
        true => {
            process::Command::new(str)
                .spawn()
                .unwrap();
        }
        false => {
            process::Command::new("/usr/bin/gnome-terminal")
                .arg("--")
                .arg("sh")
                .arg("-c")
                .arg(format!("{}; exec sh", str))
                .spawn()
                .unwrap();
        }
    }
}