pub fn winning_player(x: i32, y: i32) -> String {
    let turns = std::cmp::min(x, y / 4);
    if turns % 2 == 1 {
        "Alice".to_string()
    } else {
        "Bob".to_string()
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alice() {
        let result = winning_player(2, 7);
        assert_eq!(result, "Alice".to_string())
    }

    #[test]
    fn test_bob() {
        let result = winning_player(4, 11);
        assert_eq!(result, "Bob".to_string())
    }
}
