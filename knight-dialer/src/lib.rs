struct Solution {}

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        if n == 1 {
            return 10;
        }

        let mut matrix = [
            [0, 0, 0, 2],
            [0, 0, 1, 1],
            [0, 2, 0, 0],
            [1, 2, 0, 0],
        ];

        let mut rez = [
            [1, 0, 0, 0],
            [0, 4, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 2]
        ];

        let mut n = n - 1;
        while n != 0 {
            if n & 1 != 0 {
                rez = Self::matrix_multiplication(&rez, &matrix)
            }
            matrix = Self::matrix_multiplication(&matrix, &matrix);
            n >>= 1;
        }

        (rez.iter().flatten().sum::<i64>() % MOD) as i32
    }

    fn matrix_multiplication(a: &[[i64; 4]; 4], b: &[[i64; 4]; 4]) -> [[i64; 4]; 4] {
        let mut c = [[0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
                }
            }
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::knight_dialer(1), 10);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::knight_dialer(2), 20);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::knight_dialer(3), 46);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::knight_dialer(4), 104);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::knight_dialer(5), 240);
    }
}