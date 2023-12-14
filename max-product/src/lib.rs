struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut sorted = nums.clone();
        sorted.sort();
        let n = sorted.len();
        (sorted[n - 1] - 1) * (sorted[n - 2] - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
    }
}