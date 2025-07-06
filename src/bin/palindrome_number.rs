pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

#[allow(unused)]
fn is_palindrome_fastest(x: i32) -> bool {
    let mut clone = x;
    let mut reverse = 0;
    while clone > 0 {
        //get last digit
        let temp = clone % 10;
        //remove last digit
        clone = clone / 10;
        reverse = 10 * reverse + temp;
    }

    if reverse == x {
        return true;
    }
    return false;
}

fn main() {
    let result = is_palindrome(121);
    assert_eq!(result, true);
}

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

    #[test]
    fn is_palindrome_fastest_example1() {
        let result = is_palindrome_fastest(121);
        assert_eq!(result, true);
    }
}
