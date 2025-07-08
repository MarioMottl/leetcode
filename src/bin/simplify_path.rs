pub fn simplify_path(path: String) -> String {
    let mut stack = Vec::new();

    for part in path.split('/').filter(|s| !s.is_empty()) {
        match part {
            "." => continue,
            ".." => {
                stack.pop();
            }
            _ => stack.push(part),
        }
    }
    let output = format!("/{}", stack.join("/"));

    output
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplify_path_example1() {
        let result = simplify_path("/home/".to_owned());
        assert_eq!(result, "/home".to_string());
    }

    #[test]
    fn simplify_path_example2() {
        let result = simplify_path("/home//foo/".to_owned());
        assert_eq!(result, "/home/foo".to_string());
    }

    #[test]
    fn simplify_path_example3() {
        let result = simplify_path("/home/user/Documents/../Pictures".to_owned());
        assert_eq!(result, "/home/user/Pictures".to_string());
    }

    #[test]
    fn simplify_path_example4() {
        let result = simplify_path("/.../a/../b/c/../d/./".to_owned());
        assert_eq!(result, "/.../b/d".to_string());
    }
}
