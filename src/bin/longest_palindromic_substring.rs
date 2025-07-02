pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return "".to_string();
    }

    let mut start = 0;
    let mut max_len = 1;

    for i in 0..n {
        let len1 = expand_around_center(&chars, i as isize, i as isize);
        let len2 = expand_around_center(&chars, i as isize, i as isize + 1);
        let len = len1.max(len2);

        if len > max_len {
            max_len = len;
            start = i - (len - 1) / 2;
        }
    }

    chars[start..start + max_len].iter().collect()
}

fn expand_around_center(chars: &[char], mut left: isize, mut right: isize) -> usize {
    let n = chars.len() as isize;

    while left >= 0 && right < n && chars[left as usize] == chars[right as usize] {
        left -= 1;
        right += 1;
    }

    (right - left - 1) as usize
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_palindrome_babad() {
        let input = "babad".to_string();
        let result = longest_palindrome(input);
        assert_eq!(result, "bab".to_string());
    }

    #[test]
    fn longest_palindrome_cbbd() {
        let input = "cbbd".to_string();
        let result = longest_palindrome(input);
        assert_eq!(result, "bb".to_string());
    }
}
