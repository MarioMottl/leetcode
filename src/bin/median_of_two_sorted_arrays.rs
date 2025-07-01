#[allow(unused)]
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut joined: Vec<i32> = [nums1, nums2].concat();
    joined.sort();
    let median = joined.len() / 2;
    if joined.len() % 2 == 0 {
        return (joined[median - 1] + joined[median]) as f64 / 2 as f64;
    } else {
        return joined[median] as f64;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_total_length() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert!((result - 2.5).abs() < 1e-5, "Expected 2.5, got {}", result);
    }

    #[test]
    fn test_odd_total_length() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let result = find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 2.0);
    }
}
