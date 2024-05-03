
fn is_ok(x: usize, place: &mut [i32]) -> bool {

    for k in 1.. x {
        if place[k] == place[x] || ((place[x] - place[k]) as i32).abs() == ((x - k) as i32).abs() {
            return false;
        }
    }

    return true;
}

fn backtrack(i: usize, place: &mut [i32], n: usize, sum: &mut usize) {
    if i > n {
        *sum += 1;
    } else {
        for j in 1..n + 1 {
            place[i] = j as i32;     // 第i行下到第j列
            if is_ok(i, place) {
                backtrack(i + 1, place, n, sum);
            }
            place[i] = 0;
        }
    }
}

#[cfg(test)]
mod test{
    use crate::n_queen::backtrack;

    #[test]
    fn test() {
        let mut sum = 0;
        let n = 8;
        let mut place = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        backtrack(1, &mut place, n, &mut sum);

        assert_eq!(sum, 92);
    }
}