pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    nums.sort();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            if sum == 0 {
                result.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                right -= 1;

                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right + 1] {
                    right -= 1;
                }
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum_example1() {
        let result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]])
    }
}
