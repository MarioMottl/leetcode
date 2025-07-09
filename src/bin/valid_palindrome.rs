pub fn is_palindrome(s: String) -> bool {
    let cleaned: String = s
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    return cleaned == cleaned.chars().rev().collect::<String>();
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("race a car");
        assert_eq!(is_palindrome(s), false);
    }

    #[test]
    fn test_example_3() {
        let s = String::from(" ");
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_empty_string() {
        let s = String::from("");
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_only_special_characters() {
        let s = String::from("!!!@@@###");
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_numeric_palindrome() {
        let s = String::from("12321");
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_mixed_case_palindrome() {
        let s = String::from("MadAm");
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_non_palindrome_with_symbols() {
        let s = String::from("Hello, World!");
        assert_eq!(is_palindrome(s), false);
    }

    #[test]
    fn test_long_palindrome() {
        let s = String::from("No lemon, no melon");
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_alphanumeric_mixed() {
        let s = String::from("0P");
        assert_eq!(is_palindrome(s), false);
    }
}
