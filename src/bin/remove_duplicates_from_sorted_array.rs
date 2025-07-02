fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut i = 0;
    for j in 1..nums.len() {
        if nums[j] != nums[i] {
            i += 1;
            nums[i] = nums[j];
        }
    }

    (i + 1) as i32
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicates() {
        let mut input: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = remove_duplicates(&mut input);
        assert_eq!(result, 5)
    }
}
