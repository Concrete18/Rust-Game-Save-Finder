// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::utils;
use aho_corasick::AhoCorasick;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PossiblePath {
    pub path: String,
    pub score: i32,
}

impl PossiblePath {
    pub fn new(path: String) -> Self {
        // TODO see cleaned_path if this is needed
        let cleaned_path = path.replace('\\', "/");
        Self {
            path,
            score: score_path(cleaned_path),
        }
    }
}

fn normalize_path(input_path: &std::path::Path, target_segment: &str) -> String {
    // Split the path into components
    let mut components = input_path.components().peekable();
    let mut result_path = PathBuf::new();

    // Reconstruct the path up to and including the target segment
    while let Some(component) = components.next() {
        result_path.push(component);
        if let Some(next_component) = components.peek() {
            if next_component.as_os_str() == target_segment {
                result_path.push(next_component);
                break;
            }
        }
    }

    result_path
        .to_string_lossy()
        .to_lowercase()
        .replace('\\', "/")
        .to_string()
}

/// Finds matches for `search_string` in `path`.
fn search_path(path: String, search_string: String) -> Vec<String> {
    let mut found_paths: Vec<String> = Vec::new();
    // TODO see if walkDir can have files filtered out
    for path in WalkDir::new(path)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        // TODO check if this can be improved
        let cur_path = path.path();

        let norm_path = normalize_path(cur_path, &search_string);

        // creates path variations
        let with_space_string = &search_string.to_lowercase();
        let with_space = norm_path.contains(with_space_string);
        let without_space = norm_path.contains(&with_space_string.replace(' ', ""));
        let with_underscore = norm_path.contains(&with_space_string.replace(' ', "_"));
        // sets return value
        if with_space || without_space || with_underscore {
            println!("{norm_path}");
            // TODO make sure path is ignored if the base path already exists in found_paths
            if utils::count_occurrences(&norm_path, &search_string) != 1 {
                continue;
            }
            found_paths.push(norm_path);
        }
    }
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
        if found_paths.is_empty() {
            continue;
        }
        possible_paths.extend(found_paths);
    }
    possible_paths
}

pub fn get_directories() -> Vec<String> {
    let mut directories: Vec<String> = vec![
        "C:/Program Files (x86)/Steam/steamapps/common".to_string(),
        "C:/Users/Michael/AppData/LocalLow".to_string(),
        "C:/Users/Michael/AppData/Roaming".to_string(),
        "C:/Users/Michael/AppData/Local".to_string(),
        "C:/Users/Michael/Saved Games".to_string(),
        "C:/Users/Michael/Documents".to_string(),
    ];

    // WIP - Add a way to load custom directories
    let mut extra_dirs: Vec<String> = vec![
        "P:/SteamLibrary/steamapps/common/".to_string(),
        "D:/My Installed Games/Steam Games/steamapps/common".to_string(),
        "D:/My Documents".to_string(),
    ];
    directories.append(&mut extra_dirs);
    directories
}

#[cfg(test)]
mod search_tests {
    use super::*;
    use std::path::Path;

    /// returns dirs for tests
    pub fn find_dirs_to_check() -> Vec<String> {
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
        let string: String = "Cyberpunk 2077".to_string();
        let dirs_to_check: Vec<String> = find_dirs_to_check();
        let paths: Vec<String> = find_possible_save_paths(string, dirs_to_check);
        let answer: [&str; 3] = [
            "c:/users/michael/appdata/local/cd projekt red/cyberpunk 2077",
            "c:/users/michael/saved games/cd projekt red/cyberpunk 2077",
            "d:/my documents/cd projekt red/cyberpunk 2077",
        ];
        assert_eq!(paths, answer);
    }

    #[test]
    fn test_search_path() {
        let search_string: String = "Deep Rock Galactic".to_string();
        let path: String = "C:/Program Files (x86)/Steam/steamapps/common".to_string();
        let found_paths: Vec<String> = search_path(path, search_string);

        let answer: [&str; 1] =
            ["c:/program files (x86)/steam/steamapps/common/deep rock galactic"];

        // let test = [
        //         "c:/program files (x86)/steam/steamapps/common\\deep rock galactic",
        //         "c:/program files (x86)/steam/steamapps/common\\deep rock galactic\\engine",
        //         "c:/program files (x86)/steam/steamapps/common\\deep rock galactic\\fsd.exe",
        //     ];

        assert_eq!(found_paths, answer);
    }

    #[test]
    fn test_normalize_path() {
        const TARGET_SEGMENT: &str = "teardown";
        let path = Path::new("c:/users/michael/appdata/local/teardown\\test");
        let path_string = normalize_path(path, TARGET_SEGMENT);
        let answer = "c:/users/michael/appdata/local/teardown";
        assert_eq!(path_string, answer);
    }

    #[test]
    fn test_score_path() {
        let path = "c:/users/michael/appdata/local/teardown".to_string();
        let score = score_path(path);
        assert!(score >= 225);
    }
}
