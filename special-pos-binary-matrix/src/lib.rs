use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let row_length = mat.len();
        let col_length = mat[0].len();

        let mut row_ones: HashMap<usize, usize> = HashMap::new();
        let mut col_ones: HashMap<usize, usize> = HashMap::new();

        for row in 0..row_length {
            row_ones.insert(row, 0);
            for col in 0..col_length {
                col_ones.entry(col).or_insert(0);
                if mat[row][col] == 1 {
                    *row_ones.get_mut(&row).unwrap() += 1;
                    *col_ones.get_mut(&col).unwrap() += 1;
                }
            }
        }
        let mut specials: Vec<(usize, usize)> = Vec::new();

        for row in 0..row_length {
            for col in 0..col_length {
                if mat[row][col] == 1 && row_ones[&row] == 1 && col_ones[&col] == 1 {
                    specials.push((row, col));
                }
            }
        }
        specials.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::num_special(vec![vec![0, 0, 0, 1], vec![1, 0, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]]), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::num_special(vec![vec![0, 0, 0, 0, 0], vec![1, 0, 0, 0, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 1, 1]]), 3);
    }
}