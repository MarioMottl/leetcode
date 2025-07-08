pub fn reverse_only_letters(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let (mut left, mut right) = (0, chars.len() - 1);

    while left < right {
        if !chars[left].is_ascii_alphabetic() {
            left += 1;
        } else if !chars[right].is_ascii_alphabetic() {
            right -= 1;
        } else {
            chars.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    chars.into_iter().collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(reverse_only_letters("ab-cd".to_string()), "dc-ba");
    }

    #[test]
    fn test_mixed_case() {
        assert_eq!(
            reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba"
        );
    }

    #[test]
    fn test_with_symbols() {
        assert_eq!(
            reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1ct-eeLg=ntse-T!"
        );
    }

    #[test]
    fn test_no_letters() {
        assert_eq!(reverse_only_letters("123-456!".to_string()), "123-456!");
    }

    #[test]
    fn test_all_letters() {
        assert_eq!(reverse_only_letters("abcXYZ".to_string()), "ZYXcba");
    }
}
