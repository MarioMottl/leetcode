pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut output: Vec<Vec<i32>> = Vec::new();
    for i in 0..num_rows {
        let mut value = 1;
        let mut temp = Vec::new();
        for j in 0..=i {
            temp.push(value);
            value = value * (i - j) / (j + 1);
        }
        output.push(temp);
    }
    output
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = generate(5);
        let triangle = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(result, triangle)
    }
}
