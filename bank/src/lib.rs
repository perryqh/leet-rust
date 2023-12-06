struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut total: i32 = 0;
        let mut prev_monday = 0;
        let mut yesterday = 0;
        let mut today = 0;
        for i in 0..n {
            if i % 7 == 0 {
                // monday
                today = prev_monday + 1;
                prev_monday = today;
            } else {
                today = yesterday + 1;
            }
            yesterday = today;
            total += today;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::total_money(4), 10);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::total_money(10), 37);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::total_money(20), 96);
    }
}
