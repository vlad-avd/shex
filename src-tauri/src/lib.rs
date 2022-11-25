use std::{fs, io, process};
use std::env::var;
use std::fs::File;
use std::path::Path;
use std::process::Child;

use tauri::{AppHandle, CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTraySubmenu, Wry};

use crate::config::{Config, Executable, ShexMenuItem};

mod config;

pub fn load_config() -> Config {
    let configs_dir = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home| format!("{home}/.config")))
        .expect("Unable to determine path to 'config' directory");

    let app_config_dir = format!("{configs_dir}/shex");

    fs::create_dir_all(&app_config_dir)
        .expect(format!("Unable to create config dir by path: {config_path}").as_str());

    let config_path = format!("{config_path}/scripts_config.json");
    let config = Path::new(&config_path);

    if !config.exists() {
        match File::create(config) {
            Ok(file) => {
                serde_json::to_writer_pretty(&file, &build_demo_config())
                    .expect(format!("Failed to init config file by path: {path}").as_str());
            },
            Err(_) => panic!("Failed to create config file by path: {path}")
        }
    }

    let config = match fs::read_to_string(&config) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error while reading config file by path: {path}");
            return Config::new()
        },
    };

    match serde_json::from_str::<Config>(&config) {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Error while parsing config file by path: {path}");
            Config::new()
        },
    }
}

fn build_demo_config() -> Config {
    Config {
        items: vec![
            Box::new(
                ShexMenuItem {
                    title: String::from("submenu"),
                    background: false,
                    executable: Executable::Command {
                        body: "".to_string()
                    },
                    child_items: vec![
                        Box::new(
                            ShexMenuItem {
                                title: String::from("echo submenu"),
                                background: false,
                                executable: Executable::Command {
                                    body: String::from("echo FROM SUBMENU")
                                },
                                child_items: vec![]
                            },
                        ),
                        Box::new(
                            ShexMenuItem {
                                title: String::from("ls user"),
                                background: false,
                                executable: Executable::Command {
                                    body: String::from("ls ~")
                                },
                                child_items: vec![]
                            },
                        ),
                    ]
                },
            ),
            Box::new(
                ShexMenuItem {
                    title: String::from("echo HOME"),
                    background: false,
                    executable: Executable::Command {
                        body: String::from("echo $HOME")
                    },
                    child_items: vec![]
                },
            )
        ]
    }
}

pub fn get_event_handler()
    -> Box<dyn Fn(&AppHandle<Wry>, SystemTrayEvent) + Send + Sync + 'static> {
    // could be improved: load config only in main send here as arg
    let config = load_config().items;
    Box::new(move |app, event| -> () {
        match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => app.exit(0),
                    _item => {
                        if let Some(item) = find_item(&config, &id) {
                            handle_item_click(item)
                        }
                    }
                }
            }
            _ => {}
        }
    })
}

// for now id is the title field
pub fn find_item<'a>(items: &'a Vec<Box<ShexMenuItem>>, id: & String) -> Option<&'a Box<ShexMenuItem>> {
    let mut result = None;
    for item in items {
        if item.has_submenu() {
            result = find_item(&item.child_items, id);
        } else {
            if id.eq(&item.title) {
                return Some(&item);
            }
        }
    }
    result
}

pub fn build_tray_menu(items: &Vec<Box<ShexMenuItem>>) -> SystemTrayMenu {
    let mut menu = SystemTrayMenu::new();

    for item in items.iter() {
        let title = &item.title;

        if item.has_submenu() {
            menu = menu.add_submenu(
                SystemTraySubmenu::new(title, build_tray_menu(&item.child_items))
            );
        } else {
            // for now id is the title field
            menu = menu.add_item(CustomMenuItem::new(title, title));
        }
    }
    menu
}

pub fn handle_item_click(item: &Box<ShexMenuItem>) {
    if let Err(err) = match item.executable {
        Executable::Command { ref body } => {
            run_executable(&item, &body)
        }
        Executable::Script { ref path } => {
            run_executable(&item, &path)
        }
    } {
        eprintln!("Error while running script: {} \n error: {}", item.executable, err)
    }
}

fn run_executable(item: &ShexMenuItem, script: &str) -> io::Result<Child> {
    match item.is_background() {
        true => {
            process::Command::new(script)
                .spawn()
        }
        false => {
            process::Command::new("/usr/bin/gnome-terminal")
                .args(["--", "sh", "-c", format!("{}; exec bash", script).as_str()])
                .spawn()
        }
    }
}