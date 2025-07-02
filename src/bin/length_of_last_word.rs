#[allow(unused)]
fn length_of_last_word(s: String) -> i32 {
    let x: Vec<&str> = s.split_whitespace().collect();
    if let Some(&last_world) = x.last() {
        return last_world.len() as i32;
    }
    0
}

fn main() {}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn luffy() {
        let s = "luffy is still joyboy".to_string();
        let result = length_of_last_word(s);
        assert_eq!(result, 6);
    }
}
