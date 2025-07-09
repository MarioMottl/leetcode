pub fn can_alice_win(nums: Vec<i32>) -> bool {
    let single_digit: i32 = nums.iter().filter(|&&n| n < 10).map(|&n| n).sum();
    let double_digit: i32 = nums.iter().filter(|&&n| n >= 10).map(|&n| n).sum();
    println!("single: {}\ndouble: {}", single_digit, double_digit);
    if single_digit != double_digit {
        return true;
    } else {
        return false;
    };
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bob() {
        let result = can_alice_win(vec![1, 2, 3, 4, 10]);
        assert_eq!(result, false)
    }

    #[test]
    fn test_alice() {
        let result = can_alice_win(vec![1, 2, 3, 4, 5, 14]);
        assert_eq!(result, true)
    }
}
