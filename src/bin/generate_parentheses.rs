pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn backtrack(result: &mut Vec<String>, current: String, open: i32, close: i32, max: i32) {
        if current.len() == (max * 2) as usize {
            result.push(current);
            return;
        }

        if open < max {
            backtrack(result, format!("{}(", current), open + 1, close, max);
        }

        if close < open {
            backtrack(result, format!("{})", current), open, close + 1, max);
        }
    }
    let mut result: Vec<String> = Vec::new();
    backtrack(&mut result, String::new(), 0, 0, n);
    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_parenthesis_example1() {
        let result = generate_parenthesis(3);
        let expected = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        assert_eq!(result, expected)
    }
}
