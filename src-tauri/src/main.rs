// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use std::cmp::Reverse;
use std::process::Command;
use std::sync::Mutex;

mod app_data;
mod search;
mod utils;

// defines APP_LIST on start for use later for getting games app id's
static APP_LIST: Lazy<Mutex<Vec<app_data::App>>> =
    Lazy::new(|| Mutex::new(app_data::get_app_list()));

#[tauri::command]
fn open_path(path: String) {
    let window_path: String = path.replace('/', "\\");
    Command::new("explorer").arg(window_path).spawn().unwrap();
}

/// finds the game save directories.
#[tauri::command]
fn find_save_dirs(game_name: String) -> Vec<search::PossibleDir> {
    let directories: Vec<String> = search::get_directories();
    let dirs: Vec<String> = search::find_possible_save_dirs(game_name, directories);
    let mut scored_dirs: Vec<search::PossibleDir> = search::score_dirs(dirs);

    // Sort the scored paths by score in descending order
    scored_dirs.sort_by_key(|p: &search::PossibleDir| Reverse(p.score));

    scored_dirs
}

fn main() {
    let _ = Lazy::force(&APP_LIST);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_save_dirs, open_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
