#[allow(unused)]
fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for char in s.chars() {
        match char {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if Some(char) != stack.pop() {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_order() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_out_of_order() {
        assert_eq!(is_valid("(][){]".to_string()), false);
    }
}
