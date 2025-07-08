pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }
    let m = matrix.len();
    let mut target_row = 0;

    for i in 0..m {
        if matrix[i][0] > target {
            if i == 0 {
                return false;
            }
            target_row = i - 1;
            break;
        }
        target_row = i;
    }

    for &num in &matrix[target_row] {
        if num == target {
            return true;
        }
    }
    false
}

fn search_matrix_binary(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }
    let m = matrix.len();
    let n = matrix[0].len();

    let (mut left, mut right) = (0, m * n - 1);

    while left <= right {
        let mid = (left + right) / 2;
        let middle_value = matrix[mid / 2][mid % 2];

        if middle_value == target {
            return true;
        } else if middle_value < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_found_middle() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(search_matrix(matrix, 3), true);
    }

    #[test]
    fn test_not_found() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(search_matrix(matrix, 13), false);
    }

    #[test]
    fn test_found_last() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(search_matrix(matrix, 60), true);
    }

    #[test]
    fn test_found_first() {
        let matrix = vec![vec![1, 3, 5], vec![7, 9, 11]];
        assert_eq!(search_matrix(matrix, 1), true);
    }

    #[test]
    fn test_empty_matrix() {
        let matrix: Vec<Vec<i32>> = vec![];
        assert_eq!(search_matrix(matrix, 1), false);
    }

    #[test]
    fn test_single_element_true() {
        let matrix = vec![vec![5]];
        assert_eq!(search_matrix(matrix, 5), true);
    }

    #[test]
    fn test_single_element_false() {
        let matrix = vec![vec![5]];
        assert_eq!(search_matrix(matrix, 3), false);
    }
}
