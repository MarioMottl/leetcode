pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    use std::collections::HashSet;
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut best = 0;

    for &x in &set {
        if !set.contains(&(x - 1)) {
            let mut y = x;
            let mut len = 1;
            while set.contains(&(y + 1)) {
                y += 1;
                len += 1;
            }
            best = std::cmp::max(len, best);
        }
    }
    best
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4); // 1..=4
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9); // 0..=8
        assert_eq!(longest_consecutive(vec![]), 0);
        assert_eq!(longest_consecutive(vec![1]), 1);
        assert_eq!(longest_consecutive(vec![1, 2, 0, 1]), 3); // 0..=2, duplicates handled
        assert_eq!(
            longest_consecutive(vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6]),
            7
        ); // -1..=5
    }
}
