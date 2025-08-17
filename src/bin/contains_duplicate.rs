pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut seen = HashSet::with_capacity(nums.len());
    for number in nums {
        if seen.contains(&number) {
            return true;
        } else {
            seen.insert(number);
        }
    }
    return false;
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    }
}
