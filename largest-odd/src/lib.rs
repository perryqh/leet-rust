struct Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut last_odd: Option<usize> = None;
        for (index, ch) in num.chars().enumerate() {
            if ch.to_digit(10).unwrap() % 2 == 1 {
                last_odd = Some(index);
            }
        }
        if last_odd.is_none() {
            String::from("")
        } else {
            let last_odd = last_odd.unwrap();
            format!("{}", num.get(0..=last_odd).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::largest_odd_number("52".to_string()), "5".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::largest_odd_number("4206".to_string()), "".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::largest_odd_number("35427".to_string()), "35427".to_string());
    }
}