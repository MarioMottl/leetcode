pub fn fizz_buzz(n: i32) -> Vec<String> {
    // %3 and %5 == FizzBuzz
    // %3        == Fizz
    // %5        == Buzz
    let mut output = Vec::new();

    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            output.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            output.push("Fizz".to_string());
        } else if i % 5 == 0 {
            output.push("Buzz".to_string());
        } else {
            output.push(i.to_string());
        }
    }
    output
}

#[allow(unused)]
pub fn fizz_buzz_optimized(n: i32) -> Vec<String> {
    let mut output = Vec::with_capacity(n as usize);

    for i in 1..=n {
        let mut s = String::new();
        if i % 3 == 0 {
            s += "Fizz";
        }
        if i % 5 == 0 {
            s += "Buzz";
        }
        if s.is_empty() {
            s = i.to_string();
        }
        output.push(s);
    }

    output
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = fizz_buzz(15);
        assert_eq!(
            result,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        )
    }
}
