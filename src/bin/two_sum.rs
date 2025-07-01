use std::collections::HashMap;

#[allow(unused)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::with_capacity(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen.get(&complement) {
            return vec![j as i32, i as i32];
        }
        seen.insert(num, i);
    }
    unreachable!("No two sum found!");
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn two_sum_example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn two_sum_example_3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn two_sum_negative_numbers() {
        let nums = vec![-3, 4, 3, 90];
        let target = 0;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 2]);
    }
}
