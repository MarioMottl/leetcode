pub fn reverse(x: i32) -> i32 {
    let mut num_str: String = x.to_string();
    num_str = num_str.replace("-", "");
    if x < 0 {
        num_str.push('-');
    }
    let reversed: i32 = num_str
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    reversed
}

/// Returns 0 if the reversed integer overflows.
pub fn reverse_better(x: i32) -> i32 {
    let mut x = x;

    let mut result: i32 = 0;

    // Loop through the digits of x
    while x != 0 {
        // Take the last digit (handles negative numbers too)
        let digit = x % 10;

        // Remove the last digit from x
        x /= 10;

        /*
        Why 7 and -8?
        The max 32-bit signed integer is 2147483647 → last digit is 7
        The min is -2147483648 → last digit is -8
        */

        // Check for overflow before multiplying and adding:
        // For positive overflow
        if result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > 7) {
            return 0; // Would overflow the i32 range
        }
        // For negative overflow
        if result < i32::MIN / 10 || (result == i32::MIN / 10 && digit < -8) {
            return 0; // Would overflow the i32 range
        }

        // Safely add the digit to result
        result = result * 10 + digit;
    }

    result
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_integer_123() {
        let input: i32 = 123;
        let result = reverse(input);
        assert_eq!(result, 321);
    }

    #[test]
    fn reverse_integer_negative_123() {
        let input: i32 = 123;
        let result = reverse(input);
        assert_eq!(result, 321);
    }

    #[test]
    fn reverse_integer_better_123() {
        let input: i32 = 123;
        let result = reverse_better(input);
        assert_eq!(result, 321);
    }

    #[test]
    fn reverse_integer_negative_better_123() {
        let input: i32 = 123;
        let result = reverse_better(input);
        assert_eq!(result, 321);
    }
}
