use std::cmp::max;

// 最长公共子序列
fn lcs(x: String, y: String) -> usize {
    let len_x = x.len();
    let len_y = y.len();

    let mut dp: [[usize; 105]; 105] = [[0; 105];105];

    for i in 0..len_x {
        dp[i][0] = 0;
    }

    for j in 0..len_y {
        dp[0][j] = 0;
    }

    for i in 1..=len_x {
        for j in 1..=len_y {
            if x.as_bytes()[i - 1] == y.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    dp[len_x][len_y]
}

#[cfg(test)]
mod test {
    use crate::lcs::lcs;

    #[test]
    fn test() {
        let x = "hello World! xxx".to_string();
        let y = "Hello 1900! yyy".to_string();

        assert_eq!(lcs(x, y), 7);
    }
}
