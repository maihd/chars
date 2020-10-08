use std::collections::HashSet;

pub fn remove_duplicated_chars(content: &str) -> String {
    let chars = content.chars().collect::<HashSet<_>>();

    let mut chars_vec = chars.into_iter().collect::<Vec<_>>();
    chars_vec.sort();

    chars_vec.into_iter().collect::<String>()
}

#[test]
fn test_remove_duplicated_chars() {
    assert_eq!(remove_duplicated_chars("aaaaa"), "a");
    assert_eq!(remove_duplicated_chars("ababa"), "ab");
    assert_eq!(remove_duplicated_chars("bcaaba"), "abc");
}