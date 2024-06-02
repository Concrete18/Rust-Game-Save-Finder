// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cmp::Reverse;
use std::process::Command;

mod search;
mod utils;

#[tauri::command]
fn open_path(path: String) {
    let windows_path = path.replace('/', "\\");
    Command::new("explorer").arg(windows_path).spawn().unwrap();
}

/// finds the game saves
#[tauri::command]
fn find_game_save_paths(game_name: String) -> Vec<search::PossiblePath> {
    let directories = search::get_directories();

    // TODO add errors checking
    let cleaned_name = utils::to_alphanumeric(game_name);
    // finds possible save paths
    let paths = search::find_possible_save_paths(cleaned_name, directories);
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

// #[cfg(test)]
// mod main_tests {
//     use super::*;

// #[test]
// fn in_appdata() {
//     // get dirs
//     let dirs_to_check = search_tests::find_dirs_to_check();
//     // test vars
//     let answer = vec![search::PossiblePath::new(
//         "c:/users/michael/appdata/local/teardown".to_string(),
//     )];
//     // run test
//     let found_paths = find_game_save_paths("Teardown".to_string());
//     assert_eq!(found_paths, answer);
// }

// #[test]
// fn in_steamapps() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "Deep Rock Galactic";
//     let actual_save = "p:/steamlibrary/steamapps/common/deep rock galactic";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }

// #[test]
// fn in_saved_games() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "Cyberpunk 2077";
//     let actual_save = "c:/users/michael/saved games/cd projekt red/cyberpunk 2077";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }

// #[test]
// fn no_space() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "The Forest";
//     let actual_save = "c:/users/michael/appdata/locallow/sks/theforest";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }

// #[test]
// fn has_underscore() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "Vampire Survivor";
//     let actual_save = "c:/users/michael/appdata/roaming/vampire_survivors";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }

// #[test]
// fn contains_non_alphanumeric() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "Batman™: Arkham Knight";
//     let actual_save = "d:/my documents/wb games/batman arkham knight";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }

// #[test]
// fn outer_wilds() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "Outer Wilds";
//     let actual_save = "c:/users/michael/appdata/locallow/mobius digital/outer wilds";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }

// #[test]
// fn mini_motorway() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "Mini Motorways";
//     let actual_save = "c:/users/michael/appdata/locallow/dinosaur polo club/mini motorways";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }

// #[test]
// fn phantom_abyss() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "Phantom Abyss";
//     let actual_save = "c:/users/michael/appdata/local/phantomabyss";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }

// #[test]
// fn desperados_3() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "Desperados III";
//     let actual_save = "c:/users/michael/appdata/local/desperados iii";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }

// #[test]
// fn manifold_garden() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "Manifold Garden";
//     let actual_save = "c:/users/michael/appdata/locallow/william chyr studio/manifold garden";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }

// #[test]
// fn dishonored_2() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "Dishonored 2";
//     let actual_save = "c:/users/michael/saved games/arkane studios/dishonored2";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }

// #[test]
// fn timberborn() {
//     // get dirs
//     let dirs_to_check = find_dirs_to_check();
//     // test vars
//     let game_name = "Timberborn";
//     let actual_save = "d:/my documents/timberborn";
//     // run test
//     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
//     assert_eq!(found_path, actual_save.to_string());
// }
// }
