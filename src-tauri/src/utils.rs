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

#[cfg(test)]
mod save_search_tests {
    use super::*;

    #[test]
    fn convert_to_alphanumeric() {
        let string = "Batman™: Arkham Knight".to_string();
        let new_string = to_alphanumeric(string);
        assert_eq!(new_string, "Batman Arkham Knight".to_string());
    }
}
