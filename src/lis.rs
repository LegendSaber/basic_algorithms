use std::cmp::max;

// 最长上升子序列
fn lis(dp: &mut [usize], a: &[usize], len: usize) -> usize {
    let mut max_len = 0;

    dp[0] = 0;
    for i in 1..= len {
        dp[i] = 1;
    }

    for i in 1..= len {
        for j in 1..i {
            if a[i - 1] > a[j - 1] {
                dp[i] = max(dp[i], dp[j] + 1);
            }
        }
    }

    for i in 1..= len  {
        max_len = max(max_len, dp[i]);
    }

    max_len
}

#[cfg(test)]
mod test {
    use crate::lis::lis;
    #[test]
    fn test() {
        let mut dp = [0, 0, 0, 0, 0, 0];
        let a = [5, 4, 1, 2, 3];
        let len = a.len();

        let max_len = lis(&mut dp, &a, len);
        assert_eq!(max_len, 3);

        let mut dp = [0, 0, 0, 0, 0, 0, 0];
        let a = [4, 2, 4, 5, 3, 7];
        let len = a.len();

        let max_len = lis(&mut dp, &a, len);
        assert_eq!(max_len, 4);
    }
}
