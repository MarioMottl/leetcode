pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) {
    /*
     * i -> last actual number in nums1
     * j -> last number in nums2
     * k -> last slot in nums1
     * */
    let (mut i, mut j, mut k) = (m - 1, n - 1, m + n - 1);

    while j >= 0 {
        if i >= 0 && nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_arrays() {
        let mut input1 = vec![1, 2, 3, 0, 0, 0];
        let input2 = vec![2, 5, 6];
        merge(&mut input1, 3, &input2, 3);
        assert_eq!(input1, vec![1, 2, 2, 3, 5, 6])
    }
}
