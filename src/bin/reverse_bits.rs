pub fn reverse_bits(x: u32) -> u32 {
    x.reverse_bits()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(reverse_bits(0b00000010100101000001111010011100), 964176192);
    }
}
