pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }

    let mut one = 1; // ways to reach step n-2
    let mut two = 2; // ways to reach step n-1

    for _ in 3..=n {
        let current = one + two;
        one = two;
        two = current;
    }

    two
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn climbing_stairs() {
        let result = climb_stairs(3);
        assert_eq!(result, 3);
    }
}
