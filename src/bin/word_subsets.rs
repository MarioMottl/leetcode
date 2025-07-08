pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    let mut required = [0; 26];
    for word in &words2 {
        let mut count = [0; 26];
        for c in word.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        for i in 0..26 {
            required[i] = required[i].max(count[i]);
        }
    }

    words1
        .into_iter()
        .filter(|word| {
            let mut freq = [0; 26];
            for c in word.chars() {
                freq[(c as u8 - b'a') as usize] += 1;
            }
            (0..26).all(|i| freq[i] >= required[i])
        })
        .collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let words1 = vec!["amazon", "apple", "facebook", "google", "leetcode"]
            .into_iter()
            .map(String::from)
            .collect();

        let words2 = vec!["e", "o"].into_iter().map(String::from).collect();

        let expected: Vec<String> = vec!["facebook", "google", "leetcode"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(word_subsets(words1, words2), expected);
    }

    #[test]
    fn test_exact_letters() {
        let words1 = vec!["aabbcc", "abc", "aabbc", "abcc"]
            .into_iter()
            .map(String::from)
            .collect();
        let words2 = vec!["abc"].into_iter().map(String::from).collect();

        let expected: Vec<String> = vec!["aabbcc", "abc", "aabbc", "abcc"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(word_subsets(words1, words2), expected);
    }

    #[test]
    fn test_with_repeats() {
        let words1 = vec!["aabbcc", "abc", "aabbc", "abcc"]
            .into_iter()
            .map(String::from)
            .collect();
        let words2 = vec!["aabb", "cc"].into_iter().map(String::from).collect();

        let expected: Vec<String> = vec!["aabbcc"].into_iter().map(String::from).collect();

        assert_eq!(word_subsets(words1, words2), expected);
    }

    #[test]
    fn test_empty_words2() {
        let words1 = vec!["a", "b", "c"].into_iter().map(String::from).collect();
        let words2: Vec<String> = vec![];

        let expected: Vec<String> = vec!["a", "b", "c"].into_iter().map(String::from).collect();

        assert_eq!(word_subsets(words1, words2), expected);
    }

    #[test]
    fn test_empty_result() {
        let words1 = vec!["a", "b", "c"].into_iter().map(String::from).collect();
        let words2 = vec!["z"].into_iter().map(String::from).collect();

        let expected: Vec<String> = vec![];

        assert_eq!(word_subsets(words1, words2), expected);
    }
}
