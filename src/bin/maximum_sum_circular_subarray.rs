use std::cmp::{max, min};
use std::i32;

pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let total = nums.iter().sum::<i32>();

    let mut max_sum = i32::MIN;
    let mut cur_max = 0;

    let mut min_sum = i32::MAX;
    let mut cur_min = 0;

    for &num in &nums {
        // Kadane for max subarray
        cur_max = max(cur_max + num, num);
        max_sum = max(max_sum, cur_max);

        // Kadane for min subarray
        cur_min = min(cur_min + num, num);
        min_sum = min(min_sum, cur_min);
    }

    if max_sum < 0 {
        return max_sum;
    }

    max(max_sum, total - min_sum)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_circular() {
        let output = max_subarray_sum_circular(vec![1, -2, 3, -2]);
        assert_eq!(output, 3);
    }

    #[test]
    fn test_circular() {
        let output = max_subarray_sum_circular(vec![5, -3, 5]);
        assert_eq!(output, 10);
    }
}
