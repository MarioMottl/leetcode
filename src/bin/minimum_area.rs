fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
    /*
     * ------------------
     * |    0   1   0   \
     * |    1   0   1   |
     * ------------------
     * */

    let n = grid.len();
    let m = grid[0].len();

    let mut min_row = n;
    let mut max_row = 0usize;
    let mut min_col = m;
    let mut max_col = 0usize;

    for r in 0..n {
        for c in 0..m {
            if grid[r][c] == 1 {
                min_row = min_row.min(r);
                min_col = min_col.min(c);
                max_row = max_row.max(r);
                max_col = max_col.max(c);
            }
        }
    }

    let height = (max_row - min_row + 1) as i32;
    let width = (max_col - min_col + 1) as i32;

    height * width
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let grid = vec![vec![0, 1, 0], vec![1, 0, 1]];
        assert_eq!(minimum_area(grid), 6);
    }

    #[test]
    fn example2() {
        let grid = vec![vec![1, 0], vec![0, 0]];
        assert_eq!(minimum_area(grid), 1);
    }
}
