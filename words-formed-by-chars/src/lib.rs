use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut chars_map = Solution::string_to_map(&chars);
        let mut total_count = 0;
        for word in words {
            let cloned_chars_map = chars_map.clone();
            if Solution::good_string(&mut chars_map, &word) {
                total_count += word.len();
            }
        }

        total_count.try_into().unwrap()
    }

    fn good_string(chars_map: &mut HashMap<char, i32>, word: &str) -> bool {
        let mut cloned_chars_map = chars_map.clone();
        for ch in word.chars() {
            if let Some(count) = cloned_chars_map.get_mut(&ch) {
                if *count == 0 {
                    return false;
                } else {
                    *count -= 1;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn string_to_map(string: &str) -> HashMap<char, i32> {
        let mut map = HashMap::new();
        for char in string.chars() {
            *map.entry(char).or_insert(0) += 1;
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::count_characters(
                vec![
                    String::from("cat"),
                    String::from("bt"),
                    String::from("hat"),
                    String::from("tree")
                ],
                String::from("atach")
            ),
            6
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::count_characters(
                vec![
                    String::from("hello"),
                    String::from("world"),
                    String::from("leetcode")
                ],
                String::from("welldonehoneyr")
            ),
            10
        );
    }
}
