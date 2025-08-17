pub fn is_anagram(s: String, t: String) -> bool {
    use std::collections::HashMap;

    let mut a_seen = HashMap::new();
    let mut b_seen = HashMap::new();

    for char in s.chars() {
        *a_seen.entry(char).or_insert(0) += 1;
    }

    for char in t.chars() {
        *b_seen.entry(char).or_insert(0) += 1;
    }

    return a_seen == b_seen;
}

pub fn is_anagram_array(s: String, t: String) -> bool {
    let mut s_array: [u32; 26] = [0; 26];
    let mut t_array: [u32; 26] = [0; 26];

    for char in s.chars() {
        s_array[char.to_ascii_lowercase() as usize - 97] += 1;
    }

    for char in t.chars() {
        t_array[char.to_ascii_lowercase() as usize - 97] += 1;
    }

    assert!(s_array.len() == t_array.len());
    for index in 0..s_array.len() {
        if s_array[index] != t_array[index] {
            return false;
        }
    }
    return true;
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        assert_eq!(is_anagram_array(s, t), true);
    }

    #[test]
    fn example2() {
        let s = "car".to_string();
        let t = "rat".to_string();
        assert_eq!(is_anagram_array(s, t), false);
    }
}
