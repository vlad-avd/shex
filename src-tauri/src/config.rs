use std::path::Path;

use serde::{Deserialize, Serialize};
use crate::config::Executable::Script;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    items: Vec<ShexMenuItem>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShexMenuItem {
    title: String,
    daemon: bool,
    executable: Executable,
    child_items: Vec<Box<ShexMenuItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Executable {
    Command {body: String},
    Script {path: String},
}

impl Default for Executable {
    fn default() -> Self {
        Script {path: String::new()}
    }
}

impl ShexMenuItem {
    pub fn new() -> ShexMenuItem {
        ShexMenuItem {
            title: String::new(),
            executable: Executable::Command {body: String::new()},
            daemon: false,
            child_items: Vec::<Box<ShexMenuItem>>::new(),
        }
    }

    pub fn is_daemon(&self) -> bool {
        self.daemon
    }
}

pub fn load_config() {
    let mut config: Config = confy::load("shex", None).unwrap();
    config.items.push(ShexMenuItem{
        title: String::from("echo"),
        daemon: false,
        executable: Default::default(),
        child_items: vec![]
    });
    confy::store("shex", None, config);
    println!("{:#?}", confy::get_configuration_file_path("shex", None).unwrap());
}