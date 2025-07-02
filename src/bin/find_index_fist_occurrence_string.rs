fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.contains(&needle) {
        return haystack
            .find(&needle)
            .expect("String needs to be contained") as i32;
    } else {
        return -1;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banana() {
        let haystack = "banana".to_string();
        let needle = "nana".to_string();
        let result = str_str(haystack, needle);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_no_needle() {
        let haystack = "banana".to_string();
        let needle = "REDE".to_string();
        let result = str_str(haystack, needle);
        assert_eq!(result, -1);
    }
}
