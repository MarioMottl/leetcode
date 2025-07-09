#[allow(unused)]
pub fn minimum_cost(
    m: i32,
    n: i32,
    mut horizontal_cut: Vec<i32>,
    mut vertical_cut: Vec<i32>,
) -> i32 {
    horizontal_cut.sort_unstable_by(|a, b| b.cmp(a));
    vertical_cut.sort_unstable_by(|a, b| b.cmp(a));

    let (mut h, mut v) = (0, 0); // pointers
    let (mut h_segments, mut v_segments) = (1, 1);
    let mut total_cost = 0;

    while h < horizontal_cut.len() && v < vertical_cut.len() {
        if horizontal_cut[h] > vertical_cut[v] {
            total_cost += horizontal_cut[h] * v_segments;
            h_segments += 1;
            h += 1;
        } else {
            total_cost += vertical_cut[v] * h_segments;
            v_segments += 1;
            v += 1;
        }
    }

    // Add remaining horizontal cuts
    while h < horizontal_cut.len() {
        total_cost += horizontal_cut[h] * v_segments;
        h_segments += 1;
        h += 1;
    }

    // Add remaining vertical cuts
    while v < vertical_cut.len() {
        total_cost += vertical_cut[v] * h_segments;
        v_segments += 1;
        v += 1;
    }

    total_cost
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = minimum_cost(3, 2, vec![1, 3], vec![5]);
        assert_eq!(result, 13)
    }

    #[test]
    fn test_example2() {
        let result = minimum_cost(2, 2, vec![7], vec![4]);
        assert_eq!(result, 15)
    }
}
