struct Solution {}

impl Solution {
    const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    pub fn sort_vowels(s: String) -> String {
        let mut vowels: Vec<char> = vec![];
        let mut vowel_indexes: Vec<usize> = vec![];
        let mut result: Vec<char> = vec![];

        for (index, current_char) in s.chars().enumerate() {
            result.push(current_char);
            if Solution::VOWELS.contains(&current_char) {
                vowels.push(current_char);
                vowel_indexes.push(index);
            }
        }
        vowels.sort_unstable();
        vowels.reverse();
        for index in vowel_indexes {
            result[index] = vowels.pop().unwrap();
        }
        String::from_iter(result)
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
