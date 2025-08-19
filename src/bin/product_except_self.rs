pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut output = vec![1; n];

    // prefix product
    let mut prefix = 1;
    for i in 0..n {
        output[i] = prefix;
        prefix *= nums[i];
    }

    // suffix product
    let mut suffix = 1;
    for i in (0..n).rev() {
        output[i] *= suffix;
        suffix *= nums[i];
    }

    output
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );

        assert_eq!(product_except_self(vec![0, 0]), vec![0, 0]);
    }
}
