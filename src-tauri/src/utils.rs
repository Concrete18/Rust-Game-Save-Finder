use regex::Regex;

/// turns `string` into alphanumeric only.
pub fn to_alphanumeric(string: String) -> String {
    let mut cleaned_string = "".to_string();
    for char in string.chars() {
        if char.is_alphanumeric() || char == ' ' {
            cleaned_string.push(char)
        }
    }
    cleaned_string
}

pub fn count_occurrences(haystack: &str, needle: &str) -> usize {
    let re = Regex::new(&regex::escape(needle)).unwrap();
    re.find_iter(haystack).count()
}

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_count_occurrences() {
        let haystack = "c:/users/michael/appdata/local/teardown/teardown_mods";
        let needle = "teardown";
        let count = count_occurrences(haystack, needle);
        assert_eq!(count, 2);
    }

    #[test]
    fn convert_to_alphanumeric() {
        let string = "Batman™: Arkham Knight".to_string();
        let new_string = to_alphanumeric(string);
        assert_eq!(new_string, "Batman Arkham Knight".to_string());
    }
}
