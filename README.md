# shex

Shex is system tray app for running preconfigured sh commands and scripts built using [Tauri](https://tauri.app/).

## Instruction for

- OS: Ubuntu 20.04.5
- GUI: Gnome 41.1

## Installation

For building shex binary follow the next steps:
- Setup Tauri [Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux);
- Clone this repository;
- Open project in terminal and run: ```cd src-tauri && cargo build --release```.

## Usage

To configure tray menu open file: ```$HOME/.config/shex/scripts_config.json```.
If it is does not exist just launch app once and config file with demo menu will be created automatically.

Next customize config for yourself. It contains array of next fields:
 - *title*: title of menu item (need to be unique because used as id);
 - *background*: bool value that responds for bg or fg process will be run;
 - *executable*: describes type of executable (*command + body* or *script + path*)
 - *child_items*: array for submenu configuration.

