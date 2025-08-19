pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut key = [0u8; 26];
        // LeetCode version assumes lowercase a..z.
        for b in s.bytes() {
            key[(b - b'a') as usize] += 1;
        }
        groups.entry(key).or_default().push(s);
    }

    groups.into_values().collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = vec!["".to_string()];
        assert_eq!(group_anagrams(input), vec![vec!["".to_string()]]);

        let input = vec!["rat".to_string()];
        assert_eq!(group_anagrams(input), vec![vec!["rat".to_string()]]);
    }
}
