// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cmp::Reverse;
pub mod search;

/// finds the game saves
#[tauri::command]
fn find_game_save_paths(game_name: String, dirs_to_check: Vec<String>) -> Vec<String> {
    // TODO add errors
    let cleaned_name = search::to_alphanumeric(game_name);
    // finds possible save paths
    let paths = search::find_possible_save_paths(cleaned_name, dirs_to_check);
    let mut scored_paths: Vec<search::PossiblePath> = search::score_paths(paths);

    // Prints the scored paths
    for path in &scored_paths {
        println!("{}|{}", path.score, path.path);
    }

    // Sort the scored paths by score in descending order
    scored_paths.sort_by_key(|p| Reverse(p.score));

    // Convert the sorted paths to a vector of strings
    let result: Vec<String> = scored_paths
        .into_iter()
        .map(|p| format!("{} | {}", p.score, p.path))
        .collect();

    result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_game_save_paths])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
