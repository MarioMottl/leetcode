#[allow(unused)]
fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|x| *x != val);
    nums.len() as i32
}

fn main() {}

#[cfg(test)]
mod tests {

    #[allow(unused)]
    use super::*;
}
