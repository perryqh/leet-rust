
struct Solution {}

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut n = n;
        let mut count = 0;

        while n != 0 {
            count ^= n;
            n >>= 1;
        }

        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::minimum_one_bit_operations(0), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::minimum_one_bit_operations(3), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::minimum_one_bit_operations(6), 4);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::minimum_one_bit_operations(9), 14);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::minimum_one_bit_operations(333), 393);
    }
}