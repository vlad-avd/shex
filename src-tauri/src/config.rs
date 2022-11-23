use serde::{Deserialize, Serialize};

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
    pub(crate) daemon: bool,
    pub executable: Executable,
    pub child_items: Vec<Box<ShexMenuItem>>,
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

    pub fn has_submenu(&self) -> bool {
        !self.child_items.is_empty()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Executable {
    Command { body: String },
    Script { path: String },
}

impl Default for Executable {
    fn default() -> Self {
        Executable::Script {path: String::new()}
    }
}