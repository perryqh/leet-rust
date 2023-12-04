struct Solution {}

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        if num.len() < 3 {
            return String::from("");
        }
        let mut cur_ch = num.chars().next().unwrap().to_digit(10).unwrap();
        let mut cur_ch_cnt = 1;
        let mut max:Option<u32> = None;

        for ch in num[1..].chars() {
            let num_ch = ch.to_digit(10).unwrap();

            if num_ch == cur_ch {
                cur_ch_cnt += 1;
                if cur_ch_cnt == 3 && (max.is_none() || max.unwrap() <= num_ch){
                    max = Some(num_ch);
                }
            } else {
                cur_ch = num_ch;
                cur_ch_cnt = 1;
            }
        }
        match max {
            Some(n) => {
                format!("{}{}{}", n,n,n)
            }
            None => {
                String::from("")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1785() {
        assert_eq!(
            Solution::largest_good_integer("6777133339".to_string()),
            "777".to_string()
        );
        assert_eq!(
            Solution::largest_good_integer("2300019".to_string()),
            "000".to_string()
        );
        assert_eq!(
            Solution::largest_good_integer("4235238".to_string()),
            "".to_string()
        );
    }
}
