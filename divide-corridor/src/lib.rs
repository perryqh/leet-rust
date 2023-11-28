struct Solution {}

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        const MOD: u64 = (1e9 + 7.) as _;
        const SEAT: char = 'S';

        let mut count = 1;
        let mut seats = 0;
        let mut second_seat_index = None;

        for (index, cur_char) in corridor.char_indices() {
            if cur_char == SEAT {
                seats += 1;
                if seats == 2 {
                    second_seat_index = Some(index);
                    seats = 0;
                } else if seats == 1 && second_seat_index.is_some() {
                    count = (count * (index - second_seat_index.unwrap()) as u64) % MOD;
                }
            }
        }

        if seats == 1 || second_seat_index.is_none() {
            return 0;
        }

        count as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::number_of_ways(String::from("SSPPSPS")), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::number_of_ways(String::from("PPSPSP")), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::number_of_ways(String::from("S")), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::number_of_ways(String::from("SPSPPSSPSSSS")), 6);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::number_of_ways(String::from("SPPSSSSPPS")), 1);
        // Two ways are different if there is a position with a room divider installed in the first way but not in the second way.
    }
}
