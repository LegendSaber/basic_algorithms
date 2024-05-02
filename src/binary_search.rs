
fn binary_search(a: &[i32], begin: i32, end: i32, n: i32) -> bool {
    let mut begin = begin;
    let mut end = end;

    while begin < end {
        let mid = ((begin + end) / 2) as usize;

        if a[mid] == n {
            return true;
        } else if a[mid] > n {
            end = mid as i32 - 1;
        } else {
            begin = mid as i32 + 1;
        }
    }

    false
}

#[cfg(test)]
mod test{
    use crate::binary_search::binary_search;

    #[test]
    fn test() {
        let a = [15, 20, 35, 46, 58, 69, 74, 52, 91];
        let len = a.len() as i32;

        assert_eq!(binary_search(&a, 0, len, 58), true);
        assert_eq!(binary_search(&a, 0, len, 5), false);
        assert_eq!(binary_search(&a, 0, len, 100), false);
    }
}