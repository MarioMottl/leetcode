#[allow(unused)]
fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort_unstable();
    s.sort_unstable();

    let mut child = 0;
    let mut cookie = 0;
    let n = g.len();
    let m = s.len();

    while child < n && cookie < m {
        if s[cookie] >= g[child] {
            child += 1;
            cookie += 1;
        } else {
            cookie += 1;
        }
    }

    child as i32
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let g = vec![1, 2, 3];
        let s = vec![1, 1];
        assert_eq!(find_content_children(g, s), 1);
    }

    #[test]
    fn example2() {
        let g = vec![1, 2];
        let s = vec![1, 2, 3];
        assert_eq!(find_content_children(g, s), 2);
    }

    #[test]
    fn no_cookies() {
        let g = vec![1, 2, 3];
        let s: Vec<i32> = vec![];
        assert_eq!(find_content_children(g, s), 0);
    }

    #[test]
    fn all_satisfied() {
        let g = vec![1, 1, 1];
        let s = vec![1, 1, 1];
        assert_eq!(find_content_children(g, s), 3);
    }
}
