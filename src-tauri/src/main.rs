// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cmp::Reverse;
use std::process::Command;

mod app_data;
mod search;
mod utils;

#[tauri::command]
fn open_path(path: String) {
    let window_path: String = path.replace('/', "\\");
    Command::new("explorer").arg(window_path).spawn().unwrap();
}

/// finds the game saves
#[tauri::command]
fn find_save_dirs(game_name: String) -> Vec<search::PossibleDir> {
    let directories: Vec<String> = search::get_directories();

    // TODO add errors checking
    let cleaned_name: String = utils::to_alphanumeric(game_name);
    // finds possible save paths
    let dirs: Vec<String> = search::find_possible_save_dirs(cleaned_name, directories);
    let mut scored_dirs: Vec<search::PossibleDir> = search::score_dirs(dirs);

    // Sort the scored paths by score in descending order
    scored_dirs.sort_by_key(|p: &search::PossibleDir| Reverse(p.score));

    scored_dirs
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_save_dirs, open_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
