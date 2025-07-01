use std::{cmp::max, collections::HashSet};

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut seen: HashSet<char> = HashSet::new();
    let mut max_length = 0;
    // sliding window
    let (mut start, mut end) = (0, 0);

    while end < chars.len() {
        if seen.contains(&chars[end]) {
            seen.remove(&chars[start]);
            start += 1;
        } else {
            seen.insert(chars[end]);
            max_length = max(max_length, end - start + 1);
            end += 1;
        }
    }

    max_length as i32
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let result = length_of_longest_substring("".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_char() {
        let result = length_of_longest_substring("a".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_all_unique() {
        let result = length_of_longest_substring("abcdef".to_string());
        assert_eq!(result, 6);
    }

    #[test]
    fn test_some_repeats() {
        let result = length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(result, 3); // "abc"
    }

    #[test]
    fn test_all_same() {
        let result = length_of_longest_substring("aaaaaaa".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_middle_repeat() {
        let result = length_of_longest_substring("pwwkew".to_string());
        assert_eq!(result, 3); // "wke"
    }
}
