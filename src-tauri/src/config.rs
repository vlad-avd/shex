use std::{fs, process};
use std::env::{var, VarError};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::Executable::{Command, Script};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub items: Vec<Box<ShexMenuItem>>,
}

impl Config {
    pub fn new() -> Self {
        Self { items: Vec::<Box<ShexMenuItem>>::new() }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShexMenuItem {
    pub title: String,
    daemon: bool,
    pub executable: Executable,
    pub child_items: Vec<Box<ShexMenuItem>>,
}

impl ShexMenuItem {
    pub fn new() -> ShexMenuItem {
        ShexMenuItem {
            title: String::new(),
            executable: Command {body: String::new()},
            daemon: false,
            child_items: Vec::<Box<ShexMenuItem>>::new(),
        }
    }

    pub fn is_daemon(&self) -> bool {
        self.daemon
    }

    pub fn has_submenu(&self) -> bool {
        !self.child_items.is_empty()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Executable {
    Command {body: String},
    Script {path: String},
}

impl Default for Executable {
    fn default() -> Self {
        Script {path: String::new()}
    }
}

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
            Ok(file) => {serde_json::to_writer_pretty(&file, &get_demo_config());},
            Err(_) => println!("Ooops")
        }
    }

    let config = match fs::read_to_string(&config) {
        Ok(data) => data,
        //TODO: create a file and fill
        Err(_) => String::new(),
    };

    let mut proc = process::Command::new("/usr/bin/gnome-terminal")
        .arg("--")
        .arg("sh")
        .arg("-c")
        .arg("echo test; exec bash")
        .spawn()
        .unwrap();
    // let output = proc.wait_with_output();;
    // println!("output = {:?}", output);

    match serde_json::from_str::<Config>(&config) {
        Ok(config) => config,
        Err(_) => Config::new(),
    }

}

fn get_demo_config() -> Config {
    Config {
        items: vec![
            Box::new(
                ShexMenuItem {
                    title: "echo".to_string(),
                    daemon: false,
                    executable: Command {
                        body: String::from("echo $HOME")
                    },
                    child_items: vec![]
                },
            )
        ]
    }
}

// now id is the title field
pub fn find_item<'a>(items: &'a Vec<Box<ShexMenuItem>>, id: &'a String) -> Option<&'a Box<ShexMenuItem>> {
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