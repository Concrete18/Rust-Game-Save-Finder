use crate::utils::*;

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
        let new_string = to_alphanumeric(&string);
        assert_eq!(new_string, "Batman Arkham Knight".to_string());
    }
}
