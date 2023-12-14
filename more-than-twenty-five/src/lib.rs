use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut hash = HashMap::new();
        for el in &arr {
            let count = hash.entry(el).or_insert(0);
            *count += 1;
        }
        for (key, value) in hash {
            if value > arr.len() / 4 {
                return *key;
            }
        }
        panic!("No special integer found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
            6
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_special_integer(vec![1, 1, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]),
            3
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::find_special_integer(vec![1, 1]), 1);
    }
}
