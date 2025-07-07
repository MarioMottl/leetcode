pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let mapping = [
        "",     // 0
        "",     // 1
        "abc",  // 2
        "def",  // 3
        "ghi",  // 4
        "jkl",  // 5
        "mno",  // 6
        "pqrs", // 7
        "tuv",  // 8
        "wxyz", // 9
    ];

    let mut result = vec![];
    let mut current = String::new();

    fn backtrack(
        index: usize,
        digits: &str,
        mapping: &[&str; 10],
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        if index == digits.len() {
            result.push(current.clone());
            return;
        }

        let digit = digits.chars().nth(index).unwrap();
        let letters = mapping[digit.to_digit(10).unwrap() as usize];

        for ch in letters.chars() {
            current.push(ch);
            backtrack(index + 1, digits, mapping, current, result);
            current.pop(); // backtrack
        }
    }

    backtrack(0, &digits, &mapping, &mut current, &mut result);
    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_combinations_example1() {
        let result = letter_combinations("23".to_owned());
        assert_eq!(
            result,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}
