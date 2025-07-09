pub fn get_row(row_index: i32) -> Vec<i32> {
    let row_index: i64 = row_index as i64;
    let mut value: i64 = 1;
    let mut temp = Vec::new();
    for j in 0..=row_index {
        temp.push(value as i32);
        value = value * (row_index - j) / (j + 1);
    }
    temp
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_example2() {
        assert_eq!(get_row(0), vec![1]);
    }

    #[test]
    fn test_example3() {
        assert_eq!(get_row(1), vec![1, 1]);
    }
}
