use std::{cmp::max, collections::HashSet};

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut seen: HashSet<char> = HashSet::new();
    let mut max_length = 0;
    // sliding window
    let (mut start, mut end) = (0, 0);

    while end < chars.len() {
        if seen.contains(&chars[end]) {
            seen.remove(&chars[start]);
            start += 1;
        } else {
            seen.insert(chars[end]);
            max_length = max(max_length, end - start + 1);
            end += 1;
        }
    }

    max_length as i32
}
fn main() {}
