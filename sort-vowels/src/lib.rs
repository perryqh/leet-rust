struct Solution {}

impl Solution {
    const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    pub fn sort_vowels(s: String) -> String {
        let mut result = String::with_capacity(s.len());

        let mut reverse_vowels = Solution::reverse_sorted_vowels(&s);

        for c in s.chars() {
            if Solution::VOWELS.contains(&c) {
                result.push(reverse_vowels.pop().unwrap());
            } else {
                result.push(c);
            }
        }

        result
    }

    fn reverse_sorted_vowels(s: &str) -> Vec<char> {
        let mut all_vowels: Vec<_> = s
            .chars()
            .filter(|c| Solution::VOWELS.contains(c))
            .collect();
        all_vowels.sort_unstable();
        all_vowels.reverse();
        all_vowels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::sort_vowels(String::from("lEetcOde")),
            String::from("lEOtcede")
        );

        assert_eq!(
            Solution::sort_vowels(String::from("lYmpH")),
            String::from("lYmpH")
        );
    }
}
