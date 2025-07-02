pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    let minimal_length = strs.iter().map(|s| s.len()).min().unwrap_or(0);
    let mut prefix = String::new();

    for i in 0..minimal_length {
        let c = strs[0].as_bytes()[i];
        if strs.iter().all(|s| s.as_bytes()[i] == c) {
            prefix.push(c as char);
        } else {
            break;
        }
    }
    prefix
}

fn main() {}
