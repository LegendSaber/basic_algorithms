
fn merge(a: &mut [i32], begin: usize, end: usize, mid: usize) {
    let mut temp_array = vec![];

    let mut temp_begin = begin;
    let mut temp_mid = mid + 1;
    while temp_begin <= mid && temp_mid <= end {
        if a[temp_begin] < a[temp_mid] {
            temp_array.push(a[temp_begin]);
            temp_begin += 1;
        } else {
            temp_array.push(a[temp_mid]);
            temp_mid += 1;
        }
    }

    while temp_begin <= mid {
        temp_array.push(a[temp_begin]);
        temp_begin += 1;
    }

    while temp_mid <= end {
        temp_array.push(a[temp_mid]);
        temp_mid += 1;
    }

    let mut k = 0;
    for i in begin..end + 1 {
        a[i] = temp_array[k];
        k += 1;
    }

    println!();
}

fn merge_sort(a: &mut [i32], begin: usize, end: usize) {
    if begin < end {
        let mid = (begin + end) / 2;

        // 对左边进行归并排序
        merge_sort(a, begin, mid);
        // 对右边进行归并排序
        merge_sort(a, mid + 1, end);
        // 将排序结果合并
        merge(a, begin, end, mid);
    }
}

fn partition(a: &mut [i32], begin: usize, end: usize) -> usize {
    let flag = a[begin];
    let mut begin = begin;
    let mut end = end;

    while begin < end {
        while a[end] > flag && begin < end {
            end -= 1;
        }
        a[begin] = a[end];

        while a[begin] < flag && begin < end {
            begin += 1;
        }
        a[end] = a[begin];
    }

    a[begin] = flag;

    begin
}

fn quick_sort(a: &mut [i32], begin: usize, end: usize) {
    if begin < end {
        let q = partition(a, begin, end);
        quick_sort(a, begin, q);
        quick_sort(a, q + 1, end);
    }
}

#[cfg(test)]
mod test{
    use crate::sort::{merge_sort, quick_sort};

    #[test]
    fn test_merge() {
        let mut a = [3, 7, 6, 1, 2, 4, 5, 9, 8];
        let len = a.len() - 1;

        merge_sort(&mut a, 0, len);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_quick() {
        let mut a = [3, 7, 6, 1, 2, 4, 5, 9, 8];
        let len = a.len() - 1;

        quick_sort(&mut a, 0, len);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
