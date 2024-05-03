use std::cmp::max;

fn solve_1(n: usize, c: usize, w: &[usize], v: &[usize]) -> usize {
    let mut dp: [[usize; 105]; 105] = [[0; 105];105];

    for i in 1..= n {
        for j in 1..= c {
            if j < w[i - 1] {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = max(dp[i - 1][j - w[i - 1]] + v[i - 1], dp[i - 1][j]);
            }
        }
    }

    dp[n][c]
}

fn solve_2(n: usize, c: usize, w: &[usize], v: &[usize]) -> usize {
    let mut dp: [[usize; 105]; 105] = [[0; 105];105];

    for i in (0..n).rev() {
        for j in 0..= c {
            if j < w[i] {
                dp[i][j] = dp[i + 1][j];
            } else {
                dp[i][j] = max(dp[i + 1][j], dp[i + 1][j - w[i]] + v[i]);
            }
        }
    }

    dp[0][c]
}

#[cfg(test)]
mod test {
    use crate::bag01::{solve_1, solve_2};

    #[test]
    fn test() {
        let n = 4;
        let c = 5;
        let w = [2, 1, 3, 2];
        let v = [3, 2, 4, 2];

        assert_eq!(solve_1(n, c, &w, &v), 7);
        assert_eq!(solve_2(n, c, &w, &v), 7);
    }
}
