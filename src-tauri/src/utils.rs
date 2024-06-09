use regex::Regex;

/// turns `string` into String with alphanumeric characters only.
pub fn to_alphanumeric(string: &str) -> String {
    let mut cleaned_string: String = String::new();
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
