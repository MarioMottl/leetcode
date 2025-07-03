pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 || num_rows as usize >= s.len() {
        return s;
    }

    let chars: Vec<char> = s.chars().collect();
    let cycle_len = 2 * (num_rows - 1) as usize;
    let mut output = String::with_capacity(s.len());

    for row in 0..num_rows as usize {
        let mut i = row;
        while i < s.len() {
            output.push(chars[i]);
            let diag = i + cycle_len - 2 * row;
            if row != 0 && row != (num_rows as usize - 1) && diag < s.len() {
                output.push(chars[diag]);
            }
            i += cycle_len;
        }
    }

    output
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn zigzag_PAYPALISHIRING_3() {
        let input_str = "PAYPALISHIRING".to_string();
        let input_rows = 3;
        let result = convert(input_str, input_rows);
        assert_eq!(result, "PAHNAPLSIIGYIR".to_string())
    }

    #[allow(non_snake_case)]
    #[test]
    fn zigzag_PAYPALISHIRING_4() {
        let input_str = "PAYPALISHIRING".to_string();
        let input_rows = 4;
        let result = convert(input_str, input_rows);
        assert_eq!(result, "PINALSIGYAHRPI".to_string())
    }
}
