pub fn hamming_weight(n: i32) -> i32 {
    n.count_ones() as i32
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(hamming_weight(11), 3);
    }
}
