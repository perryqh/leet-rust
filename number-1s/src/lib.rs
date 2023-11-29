struct Solution {}

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut count = 0;
        let mut n = n;

        while n != 0 {
            count += 1;
            n &= n - 1; // removes the least significant 1-bit
        }

        count.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::hammingWeight(0b00000000000000000000000000001011), 3);
        assert_eq!(Solution::hammingWeight(0b00000000000000000000000010000000), 1);
        assert_eq!(Solution::hammingWeight(0b11111111111111111111111111111101), 31);
        assert_eq!(Solution::hammingWeight(0b00000000000000000000000000000000), 0);
    }
}
