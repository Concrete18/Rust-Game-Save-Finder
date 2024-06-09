use crate::search::*;

#[cfg(test)]
mod search_tests {
    use super::*;

    #[test]
    fn test_check_user_data() {
        let path_string: Result<String, String> = check_user_data(1364780);
        const ANSWER: &str = "c:/program files (x86)/steam/userdata/22360464/1364780";
        assert_eq!(path_string.unwrap(), ANSWER);
    }

    #[test]
    fn test_remove_duplicate_dirs() {
        let directories: Vec<String> = vec![
            "p:/steamlibrary/steamapps/common".to_string(),
            "p:/steamlibrary/steamapps/common/test1".to_string(),
            "p:/steamlibrary/steamapps/common/test1/test2".to_string(),
        ];
        let final_dirs: Vec<String> = remove_duplicate_dirs(directories);

        let answer: Vec<String> = vec!["p:/steamlibrary/steamapps/common".to_string()];
        assert_eq!(final_dirs, answer);
    }

    #[test]
    fn test_search_dir() {
        let search_string: String = "deep rock galactic".to_string();
        let directory: String = "p:/steamlibrary/steamapps/common".to_string();
        let found_dirs: Vec<String> = search_dir(directory, &search_string);

        let answer: String = "p:/steamlibrary/steamapps/common/deep rock galactic".to_string();
        assert!(found_dirs.contains(&answer))
    }

    #[test]
    fn test_score_dir() {
        let directory: String = "c:/users/michael/appdata/local/teardown".to_string();
        let score: i32 = score_dir(&directory);
        assert!(score >= 225);
    }
}
