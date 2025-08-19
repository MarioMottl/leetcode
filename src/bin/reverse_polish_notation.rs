pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    /*
     * Input: ["2","1","+","3","*"]
     * Iterate until the first *+-/
     * take index of sign -1 and -2 do the calculation
     * save calculation at index sign remove index of -1 and -2
     * call function again until tokens.len() == 1
     * */

    let mut st = Vec::with_capacity(tokens.len());
    for t in tokens {
        match t.as_str() {
            "+" => {
                let b = st.pop().unwrap();
                let a = st.pop().unwrap();
                st.push(a + b);
            }
            "-" => {
                let b = st.pop().unwrap();
                let a = st.pop().unwrap();
                st.push(a - b);
            }
            "*" => {
                let b = st.pop().unwrap();
                let a = st.pop().unwrap();
                st.push(a * b);
            }
            "/" => {
                let b = st.pop().unwrap();
                let a = st.pop().unwrap();
                st.push(a / b); // truncates toward zero
            }
            _ => st.push(t.parse::<i32>().unwrap()),
        }
    }
    st.pop().unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let tokens = vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ];
        assert_eq!(eval_rpn(tokens), 9)
    }
}
