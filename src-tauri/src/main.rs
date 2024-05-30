// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cmp::Reverse;
use std::process::Command;
pub mod search;

#[tauri::command]
fn open_path(path: String) {
    let cleaned_path = path.replace('/', "\\");
    Command::new("explorer").arg(cleaned_path).spawn().unwrap();
}

/// finds the game saves
#[tauri::command]
fn find_game_save_paths(
    game_name: String,
    dirs_to_check: Vec<String>,
) -> Vec<search::PossiblePath> {
    // TODO add errors
    let cleaned_name = search::to_alphanumeric(game_name);
    // finds possible save paths
    let paths = search::find_possible_save_paths(cleaned_name, dirs_to_check);
    let mut scored_paths: Vec<search::PossiblePath> = search::score_paths(paths);

    // Sort the scored paths by score in descending order
    scored_paths.sort_by_key(|p| Reverse(p.score));

    scored_paths
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_game_save_paths, open_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
