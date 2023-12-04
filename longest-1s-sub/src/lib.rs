use std::io::BufRead;

struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        nums.split(|&el| el == 0) // split the array into slices of 1s
            .map(|ones_slice| ones_slice.len() as i32) // convert the slices to their lengths
            .scan(-1, |prev, curr| {
                // add up the previous and current slice lengths, which is equivalent to dropping the middle 0
                let output = *prev + curr;
                *prev = curr;
                Some(output)
            })
            .max() // max of sums of adjacent slices
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
            5
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1]), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::longest_subarray(vec![1, 1, 0, 0, 1, 1, 1, 0, 1]),
            4
        );
        // 1,1,1,0,1,0,1,1,0
        // 1 1 1 s 1,s
        //       0 1,s,1,1,
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_subarray(vec![0, 0, 0]), 0);
    }
}
