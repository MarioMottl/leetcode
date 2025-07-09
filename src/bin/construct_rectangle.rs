pub fn construct_rectangle(area: i32) -> Vec<i32> {
    let mut w = (area as f64).sqrt() as i32;
    println!("{}", w);
    while area % w != 0 {
        w -= 1;
    }
    vec![area / w, w]
}

fn main() {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example1() {
        let result = construct_rectangle(4);
        assert_eq!(result, vec![2, 2])
    }
}
