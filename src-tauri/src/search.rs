// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aho_corasick::AhoCorasick;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
pub struct PossiblePath {
    pub path: String,
    pub score: i32,
}

impl PossiblePath {
    fn new(path: String) -> Self {
        let cleaned_path = path.replace('\\', "/");
        Self {
            path,
            score: score_path(cleaned_path),
        }
    }
}

/// Finds matches for `search_string` in `path`.
fn search_path(path: String, search_string: String) -> Vec<String> {
    let mut found_paths: Vec<String> = Vec::new();
    for path in WalkDir::new(path)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let cur_path = String::from(path.path().to_string_lossy()).to_lowercase();
        // creates path variations
        let with_space_string = &search_string.to_lowercase();
        let with_space = cur_path.contains(with_space_string);
        let without_space = cur_path.contains(&with_space_string.replace(' ', ""));
        let with_underscore = cur_path.contains(&with_space_string.replace(' ', "_"));
        // sets return value
        if with_space || without_space || with_underscore {
            // TODO make sure path is ignored if the base path already exists in found_paths
            found_paths.push(cur_path);
        }
    }
    // example of duplicate places to check
    // let test = [
    //         "c:/program files (x86)/steam/steamapps/common\\deep rock galactic",
    //         "c:/program files (x86)/steam/steamapps/common\\deep rock galactic\\engine",
    //         "c:/program files (x86)/steam/steamapps/common\\deep rock galactic\\fsd.exe",
    //     ];
    found_paths
}

/// Scores path points based on occurrences of
fn score_path(path: String) -> i32 {
    // positive scoring array
    const SCORE_POS: [&str; 20] = [
        "autosave",
        "quicksave",
        "manualsave",
        "saveslot",
        "SteamSaves",
        "Backup",
        "sav.",
        ".sav",
        "config.ini",
        "userdata",
        "steam_autocloud",
        "Player.log",
        "Player-prev.log",
        "output_log.txt",
        "slot",
        "screenshot",
        "save",
        ".zip",
        ".dat",
        "profile",
    ];
    let ac_pos = AhoCorasick::new(SCORE_POS).unwrap();
    // negative scoring array
    const SCORE_NEG: [&str; 4] = ["nvidia", ".exe", ".dll", ".assets"];
    let ac_neg = AhoCorasick::new(SCORE_NEG).unwrap();
    // get total score
    let mut total_score = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let cur_path = String::from(entry.path().to_string_lossy()).to_lowercase();

        for _match in ac_pos.find_iter(&cur_path) {
            total_score += 25;
        }

        for _match in ac_neg.find_iter(&cur_path) {
            total_score -= 30;
        }
    }
    total_score
}

// Define a struct to hold the string and its associated number

pub fn score_paths(paths: Vec<String>) -> Vec<PossiblePath> {
    let mut scored_paths = Vec::new();
    for path in &paths {
        if path.contains('.') {
            continue;
        }
        let scored_path = PossiblePath::new(path.to_string());
        scored_paths.push(scored_path);
    }
    scored_paths
}

// TODO add function that searches for saves with games app id

/// Finds possible save paths for `search_string` within `dirs_to_check`.
pub fn find_possible_save_paths(search_string: String, dirs_to_check: Vec<String>) -> Vec<String> {
    let mut possible_paths = Vec::new();
    for dir in dirs_to_check {
        let found_paths = search_path(dir, search_string.to_string());
        if !found_paths.is_empty() {
            possible_paths.extend(found_paths);
        }
    }
    possible_paths
}

// TODO move tests to their own folder
#[cfg(test)]
mod save_search_tests {
    use super::*;

    /// returns dirs for tests
    fn find_dirs_to_check() -> Vec<String> {
        let dirs_to_check = vec![
            "C:/Program Files (x86)/Steam/steamapps/common".to_string(),
            "P:/SteamLibrary/steamapps/common/".to_string(),
            "C:/Users/Michael/AppData/LocalLow".to_string(),
            "C:/Users/Michael/AppData/Roaming".to_string(),
            "C:/Users/Michael/AppData/Local".to_string(),
            "C:/Users/Michael/Saved Games".to_string(),
            "C:/Users/Michael/Documents".to_string(),
            "D:/My Installed Games/Steam Games/steamapps/common".to_string(),
            "D:/My Documents".to_string(),
        ];
        dirs_to_check
    }

    #[test]
    fn find_possible_save_paths_test() {
        let string = "Cyberpunk 2077".to_string();
        let dirs_to_check = find_dirs_to_check();
        let paths = find_possible_save_paths(string, dirs_to_check);
        let answer = [
            "c:/users/michael/appdata/local\\cd projekt red\\cyberpunk 2077",
            "c:/users/michael/saved games\\cd projekt red\\cyberpunk 2077",
            "d:/my documents\\cd projekt red\\cyberpunk 2077",
        ];
        assert_eq!(paths, answer);
    }

    #[test]
    fn scoring_test() {
        let path = "c:/users/michael/appdata/local/teardown".to_string();
        let score = score_path(path);
        assert!(score >= 225);
    }

    // #[test]
    // fn in_appdata() {
    //     // get dirs
    //     let dirs_to_check = find_dirs_to_check();
    //     // test vars
    //     let game_name = "Teardown";
    //     let actual_save = "c:/users/michael/appdata/local/teardown";
    //     // run test
    //     let found_path = find_game_save_paths(game_name.to_string(), dirs_to_check);
    //     assert_eq!(found_path, actual_save.to_string());
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
}
