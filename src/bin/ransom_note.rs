#[allow(unused)]
fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut counts = [0; 26];

    for c in magazine.chars() {
        counts[c as usize - 'a' as usize] += 1;
    }

    for c in ransom_note.chars() {
        let idx = c as usize - 'a' as usize;
        if counts[idx] == 0 {
            return false;
        }
        counts[idx] -= 1;
    }

    true
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(can_construct("a".to_string(), "b".to_string()), false);
    }

    #[test]
    fn example2() {
        assert_eq!(can_construct("aa".to_string(), "ab".to_string()), false);
    }

    #[test]
    fn example3() {
        assert_eq!(can_construct("aa".to_string(), "aab".to_string()), true);
    }
}
