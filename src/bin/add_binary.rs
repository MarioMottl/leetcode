#[allow(unused)]
fn add_binary(a: String, b: String) -> String {
    let (mut i, mut j) = (a.len() as i32 - 1, b.len() as i32 - 1);
    let (a_bytes, b_bytes) = (a.as_bytes(), b.as_bytes());
    let mut carry = 0;
    let mut result = String::new();

    while i >= 0 || j >= 0 || carry > 0 {
        let mut sum = carry;
        if i >= 0 {
            sum += (a_bytes[i as usize] - b'0') as u8;
            i -= 1;
        }
        if j >= 0 {
            sum += (b_bytes[j as usize] - b'0') as u8;
            j -= 1;
        }

        result.push(char::from(b'0' + (sum % 2)));
        carry = sum / 2;
    }

    result.chars().rev().collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_binary_example1() {
        let input_a = "1010".to_string();
        let input_b = "1011".to_string();

        let result = add_binary(input_a, input_b);
        assert_eq!("10101".to_string(), result);
    }
}
