pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_example1() {
        let result = is_palindrome(-121);
        assert_eq!(result, false);
    }

    #[test]
    fn is_palindrome_example2() {
        let result = is_palindrome(10);
        assert_eq!(result, false);
    }

    #[test]
    fn is_palindrome_example3() {
        let result = is_palindrome(121);
        assert_eq!(result, true);
    }
}
