struct Solution;

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut matches = 0;
        let mut teams_left = n;
        while teams_left != 1 {
            let bye = teams_left % 2;
            teams_left = teams_left / 2;
            matches += teams_left;
            teams_left += bye;
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_played_test() {
        assert_eq!(Solution::number_of_matches(7), 6);
        assert_eq!(Solution::number_of_matches(14), 13);
    }
}
