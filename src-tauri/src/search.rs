// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::utils;
use aho_corasick::AhoCorasick;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PossibleDir {
    pub directory: String,
    pub score: i32,
}

impl PossibleDir {
    pub fn new(directory: String) -> Self {
        let score = score_dir(&directory);
        Self { directory, score }
    }
}

/// Scores based on occurrences of contents of the directory
fn score_dir(directory: &String) -> i32 {
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
    let ac_pos: AhoCorasick = AhoCorasick::new(SCORE_POS).unwrap();
    // negative scoring array
    const SCORE_NEG: [&str; 4] = ["nvidia", ".exe", ".dll", ".assets"];
    let ac_neg: AhoCorasick = AhoCorasick::new(SCORE_NEG).unwrap();
    // get total score
    let mut total_score = 0;
    for dir in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let cur_path: String = String::from(dir.path().to_string_lossy()).to_lowercase();

        for _match in ac_pos.find_iter(&cur_path) {
            total_score += 25;
        }

        for _match in ac_neg.find_iter(&cur_path) {
            total_score -= 30;
        }
    }
    total_score
}

fn normalize_path(input_path: &std::path::Path, target_segment: &str) -> String {
    // split the path into components
    let mut components: std::iter::Peekable<std::path::Components> =
        input_path.components().peekable();
    let mut result_path: PathBuf = PathBuf::new();

    // reconstruct the path up to and including the target segment
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
fn search_dir(directory: String, search_string: String) -> Vec<String> {
    let mut found_dirs: Vec<String> = Vec::new();
    // TODO see if walkDir can have files filtered out
    for directory in WalkDir::new(directory)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let norm_path: String = normalize_path(directory.path(), &search_string);
        // creates directory variations
        let with_space_string: &String = &search_string.to_lowercase();
        let with_space: bool = norm_path.contains(with_space_string);
        let without_space: bool = norm_path.contains(&with_space_string.replace(' ', ""));
        let with_underscore: bool = norm_path.contains(&with_space_string.replace(' ', "_"));
        // sets return value
        if with_space || without_space || with_underscore {
            println!("{norm_path}");
            if utils::count_occurrences(&norm_path, with_space_string) != 1 {
                continue;
            }
            found_dirs.push(norm_path);
        }
    }
    // TODO remove paths that contain another path as a substring

    found_dirs
}

pub fn score_dirs(dirs: Vec<String>) -> Vec<PossibleDir> {
    let mut scored_dirs = Vec::new();
    for dir in &dirs {
        if dir.contains('.') {
            continue;
        }
        let scored_directory = PossibleDir::new(dir.to_string());
        scored_dirs.push(scored_directory);
    }
    scored_dirs
}

// TODO add function that searches for saves with games app id

/// Finds possible save paths for `search_string` within `dirs_to_check`.
pub fn find_possible_save_dirs(search_string: String, dirs_to_check: Vec<String>) -> Vec<String> {
    let mut possible_dirs: Vec<String> = Vec::new();
    for directory in dirs_to_check {
        let found_dirs: Vec<String> = search_dir(directory, search_string.to_string());
        if found_dirs.is_empty() {
            continue;
        }
        possible_dirs.extend(found_dirs);
    }
    possible_dirs
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
    pub fn get_test_dirs() -> Vec<String> {
        let dirs_to_check: Vec<String> = vec![
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
        let dirs_to_check: Vec<String> = get_test_dirs();
        let dirs: Vec<String> = find_possible_save_dirs(string, dirs_to_check);
        let answer: [&str; 3] = [
            "c:/users/michael/appdata/local/cd projekt red/cyberpunk 2077",
            "c:/users/michael/saved games/cd projekt red/cyberpunk 2077",
            "d:/my documents/cd projekt red/cyberpunk 2077",
        ];
        assert_eq!(dirs, answer);
    }

    #[test]
    fn test_search_dir() {
        let search_string: String = "deep rock galactic".to_string();
        let directory: String = "p:/steamlibrary/steamapps/common".to_string();
        let found_dirs: Vec<String> = search_dir(directory, search_string);

        let answer: [&str; 1] = ["p:/steamlibrary/steamapps/common/deep rock galactic"];

        // let test = [
        //     "p:/steamlibrary/steamapps/common/deep rock galactic",
        //     "p:/steamlibrary/steamapps/common/deep rock galactic/engine",
        //     "p:/steamlibrary/steamapps/common/deep rock galactic/fsd",
        // ];

        assert_eq!(found_dirs, answer);
    }

    #[test]
    fn test_normalize_path() {
        const TARGET_SEGMENT: &str = "teardown";
        let path: &Path = Path::new("c:/users/michael/appdata/local/teardown\\test");
        let path_string: String = normalize_path(path, TARGET_SEGMENT);
        let answer: &str = "c:/users/michael/appdata/local/teardown";
        assert_eq!(path_string, answer);
    }

    #[test]
    fn test_score_dir() {
        let directory: String = "c:/users/michael/appdata/local/teardown".to_string();
        let score: i32 = score_dir(&directory);
        assert!(score >= 225);
    }
}
