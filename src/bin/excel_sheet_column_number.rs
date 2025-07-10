pub fn title_to_number(column_title: String) -> i32 {
    column_title
        .chars()
        .fold(0, |acc, c| acc * 26 + (c as u8 - b'A' + 1) as i32)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(title_to_number("A".to_string()), 1)
    }

    #[test]
    fn test_ab() {
        assert_eq!(title_to_number("AB".to_string()), 28)
    }

    #[test]
    fn test_zy() {
        assert_eq!(title_to_number("ZY".to_string()), 701)
    }
}
