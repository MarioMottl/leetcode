pub fn is_power_of_two(n: i32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(is_power_of_two(16), true)
    }

    #[test]
    fn example_2() {
        assert_eq!(is_power_of_two(1), true)
    }

    #[test]
    fn example_3() {
        assert_eq!(is_power_of_two(3), false)
    }

    #[test]
    fn example_4() {
        assert_eq!(is_power_of_two(5), false)
    }
}
