pub fn min_changes(n: i32, k: i32) -> i32 {
    if k & !n != 0 {
        return -1;
    }

    (n & !k).count_ones() as i32
}

fn main() {}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
}
