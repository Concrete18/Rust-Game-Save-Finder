#![allow(dead_code)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app_data;
use crate::utils;
use crate::APP_LIST;

use aho_corasick::AhoCorasick;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;
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

/// Scores based on occurrences of contents of the directory.
pub fn score_dir(directory: &String) -> i32 {
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

/// Returns a new vector with duplicate directories removed.
/// Directories containting a smaller directrory as a substring count as duplicate.
pub fn remove_duplicate_dirs(mut directories: Vec<String>) -> Vec<String> {
    directories.sort();
    let mut unique_dirs: HashSet<String> = HashSet::new();

    let mut result: Vec<String> = Vec::new();
    for dir in directories {
        let mut is_subdir: bool = false;
        for existing in &unique_dirs {
            if dir.starts_with(existing) {
                is_subdir = true;
                break;
            }
        }
        if !is_subdir {
            unique_dirs.insert(dir.clone());
            result.push(dir);
        }
    }

    result
}

/// Finds matches for `search_string` in `path`.
pub fn search_dir(directory: String, game_name: &str) -> Vec<String> {
    let mut found_dirs: Vec<String> = Vec::new();
    for directory in WalkDir::new(directory)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let path_string: String = directory
            .path()
            .to_string_lossy()
            .to_lowercase()
            .replace('\\', "/")
            .to_string();

        // creates directory variations
        let with_space_string: &String = &game_name.to_lowercase();
        let with_space: bool = path_string.contains(with_space_string);
        let without_space: bool = path_string.contains(&with_space_string.replace(' ', ""));
        let with_underscore: bool = path_string.contains(&with_space_string.replace(' ', "_"));
        // sets return value
        if with_space || without_space || with_underscore {
            if utils::count_occurrences(&path_string, with_space_string) != 1 {
                continue;
            }
            found_dirs.push(path_string);
        }
    }
    // TODO check if this needs changes due to removing of deeper and more accurate folder suggestions
    remove_duplicate_dirs(found_dirs)
}

/// Scores directories based on contents to determine if it could be a game save directory.
pub fn score_dirs(dirs: Vec<String>) -> Vec<PossibleDir> {
    let mut scored_dirs: Vec<PossibleDir> = Vec::new();
    for dir in &dirs {
        if dir.contains('.') {
            continue;
        }
        let scored_directory: PossibleDir = PossibleDir::new(dir.to_string());
        scored_dirs.push(scored_directory);
    }
    scored_dirs
}

pub fn check_user_data(app_id: u32) -> Result<String, String> {
    let app_data_dir: &str = "c:/Program Files (x86)/Steam/userdata";
    for dir in WalkDir::new(app_data_dir)
        .max_depth(2)
        .into_iter()
        .filter_map(|e: Result<walkdir::DirEntry, walkdir::Error>| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let dir_path = dir
            .path()
            .to_string_lossy()
            .to_string()
            .to_lowercase()
            .replace('\\', "/");
        if dir_path.contains(&app_id.to_string()) {
            return Ok(dir_path);
        }
    }
    Err("Nothing Found".to_string())
}

/// Finds possible save paths for `search_string` within `directories`.
pub fn find_possible_save_dirs(game_name: String, directories: Vec<String>) -> Vec<String> {
    let cleaned_name: String = utils::to_alphanumeric(&game_name);
    let mut possible_dirs: Vec<String> = Vec::new();
    for directory in directories {
        // skip if path does not exist
        if !Path::new(&directory).exists() {
            continue;
        }
        let found_dirs: Vec<String> = search_dir(directory, &cleaned_name);
        // skip if directory is empty
        if found_dirs.is_empty() {
            continue;
        }
        possible_dirs.extend(found_dirs);
    }

    // TODO check what happens when it fails
    let app_list = APP_LIST.lock().unwrap().to_vec();

    // checks user data
    if !app_list.is_empty() {
        if let Ok(app_id) = app_data::get_app_id(game_name, app_list) {
            if let Ok(directory) = check_user_data(app_id) {
                possible_dirs.push(directory)
            }
        }
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
