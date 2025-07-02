#[allow(unused)]
fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in (0..digits.len()).rev() {
        if digits[i] < 9 {
            digits[i] += 1;
            return digits;
        }
        digits[i] = 0;
    }
    digits.insert(0, 1);
    digits
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_one_example1() {
        let digits = vec![1, 2, 3];
        let result = plus_one(digits);
        assert_eq!(result, vec![1, 2, 4]);
    }

    #[test]
    fn plus_one_example2() {
        let digits = vec![9];
        let result = plus_one(digits);
        assert_eq!(result, vec![1, 0]);
    }
}
