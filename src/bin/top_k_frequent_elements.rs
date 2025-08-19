pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut freq = HashMap::new();

    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }
    let mut vec: Vec<_> = freq.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));
    vec.iter()
        .take(k as usize)
        .map(|&(&k, _)| k.clone())
        .collect()
}

fn main() {}
