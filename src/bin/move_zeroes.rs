pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut insert_pos = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[insert_pos] = nums[i];
            insert_pos += 1;
        }
    }

    for i in insert_pos..nums.len() {
        nums[i] = 0;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_example2() {
        let mut nums = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_no_zeroes() {
        let mut nums = vec![1, 2, 3];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_all_zeroes() {
        let mut nums = vec![0, 0, 0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0, 0]);
    }

    #[test]
    fn test_zeros_in_middle() {
        let mut nums = vec![4, 0, 5, 0, 6];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![4, 5, 6, 0, 0]);
    }

    #[test]
    fn test_alternating_zeroes() {
        let mut nums = vec![0, 1, 0, 2, 0, 3];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 0, 0, 0]);
    }
}
