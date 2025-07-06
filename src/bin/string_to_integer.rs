pub fn my_atoi(s: String) -> i32 {
    let s = s.trim_start();
    if s.is_empty() {
        return 0;
    }

    let mut sign = 1;
    let mut index = 0;
    let chars: Vec<char> = s.chars().collect();

    if chars[index] == '-' {
        sign = -1;
        index += 1;
    } else if chars[index] == '+' {
        index += 1;
    }

    let mut result: i64 = 0;

    while index < chars.len() && chars[index].is_ascii_digit() {
        let digit = (chars[index] as u8 - b'0') as i64;
        result = result * 10 + digit;

        if sign * result < i32::MIN as i64 {
            return i32::MIN;
        } else if sign * result > i32::MAX as i64 {
            return i32::MAX;
        }

        index += 1;
    }

    (sign * result) as i32
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atoi_example1() {
        let result = my_atoi(" -042".to_owned());
        assert_eq!(result, -42);
    }

    #[test]
    fn atoi_example2() {
        let result = my_atoi("1337c0d3".to_owned());
        assert_eq!(result, 1337);
    }
}
