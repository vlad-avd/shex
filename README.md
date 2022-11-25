# shex

Shex is system tray app built using [Tauri](https://tauri.app/) for running preconfigured sh commands and scripts.

## Instruction for

- OS: Ubuntu 20.04.5
- GUI: Gnome 41.1

## Installation

For building shex binary follow the next steps:
- Clone this repository;
- Setup Tauri [Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux);
- Install tauri-cli by running: ```cargo install tauri-cli```;
- Open project in terminal and run: ```cd src-tauri && cargo build --release```.

To launch app, execute: ```cargo tauri dev``` or just run binary by path: ```/src-tauri/target/release/shex```

## Usage

To configure tray menu, open the file: ```$HOME/.config/shex/scripts_config.json```.
If it does not exist, just launch app once and config file with demo menu will be created automatically.

Next, customize config for yourself. It contains array of next fields:
 - *title*: title of menu item (need to be unique because used as id);
 - *background*: bool value that responds for bg or fg process will be run;
 - *executable*: describes type of executable (*command + body* or *script + path*)
 - *child_items*: array for submenu configuration.

You can configure service for app autostart when os start up:
!TODO

For restarting app on the fly when config file changed, follow next steps:
!TODO