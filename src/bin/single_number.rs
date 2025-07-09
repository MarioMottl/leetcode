use std::collections::HashMap;

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut freq = HashMap::new();

    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    for (key, val) in freq {
        if val == 1 {
            return key;
        }
    }

    0
}

#[allow(unused)]
fn single_number_xor(nums: Vec<i32>) -> i32 {
    let mut result = 0;

    for num in nums {
        result ^= num;
    }

    result
}

fn main() {}

#[cfg(test)]
mod tests {

    #[allow(unused)]
    use super::*;
}
