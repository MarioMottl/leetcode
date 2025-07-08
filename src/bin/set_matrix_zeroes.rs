pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut first_row_zero = false;
    let mut first_col_zero = false;

    for j in 0..n {
        if matrix[0][j] == 0 {
            first_row_zero = true;
            break;
        }
    }

    for i in 0..m {
        if matrix[i][0] == 0 {
            first_col_zero = true;
            break;
        }
    }

    for i in 1..m {
        for j in 1..n {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }

    for i in 1..m {
        for j in 1..n {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }

    if first_row_zero {
        for j in 0..n {
            matrix[0][j] = 0;
        }
    }

    if first_col_zero {
        for i in 0..m {
            matrix[i][0] = 0;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_zeroes_example1() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut matrix);
        let expected = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        assert_eq!(matrix, expected);
    }

    #[test]
    fn set_zeroes_example2() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut matrix);
        let expected = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        assert_eq!(matrix, expected);
    }

    #[test]
    fn set_zeroes_no_zeros() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        let expected = matrix.clone();
        set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn set_zeroes_all_zeros() {
        let mut matrix = vec![vec![0, 0], vec![0, 0]];
        let expected = vec![vec![0, 0], vec![0, 0]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn set_zeroes_one_row() {
        let mut matrix = vec![vec![1, 0]];
        let expected = vec![vec![0, 0]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
