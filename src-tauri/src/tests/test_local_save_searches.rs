use crate::search::*;

/// returns dirs for tests.
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
    let game_name: String = "Cyberpunk 2077".to_string();
    let directories: Vec<String> = get_test_dirs();

    let possible_dirs: Vec<String> = find_possible_save_dirs(game_name, directories);
    let answer: String = "c:/users/michael/appdata/local/cd projekt red/cyberpunk 2077".to_string();
    assert_eq!(possible_dirs[0], answer);
}

// TODO add more tests
