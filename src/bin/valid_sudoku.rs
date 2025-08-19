type Board = Vec<Vec<char>>;

pub fn is_valid_sudoku(board: &Board) -> bool {
    let mut rows = [0u16; 9];
    let mut cols = [0u16; 9];
    let mut boxes = [0u16; 9];

    for r in 0..9 {
        for c in 0..9 {
            let ch = board[r][c];
            if ch == '.' {
                continue;
            }
            if ch < '1' || ch > '9' {
                return false;
            } // only 1..=9 allowed

            let d = (ch as u8 - b'1') as usize; // 0..=8
            let mask = 1u16 << d;
            let b = (r / 3) * 3 + (c / 3); // box index 0..=8

            if (rows[r] & mask) != 0 || (cols[c] & mask) != 0 || (boxes[b] & mask) != 0 {
                return false;
            }
            rows[r] |= mask;
            cols[c] |= mask;
            boxes[b] |= mask;
        }
    }
    true
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board1() {
        let board: Board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(&board), true);
    }

    #[test]
    fn board2() {
        let board: Board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(is_valid_sudoku(&board), false);
    }
}
