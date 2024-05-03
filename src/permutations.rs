
fn exchange(a: &mut [i32], x: usize, y: usize) {
    let temp = a[x];
    a[x] = a[y];
    a[y] = temp;
}

fn permutations(a: &mut [i32], begin: usize, end: usize) {
    if begin == end - 1 {
        println!("{:?}", a);
    } else {
        for i in begin..end  {
            exchange(a, begin, i);
            permutations(a, begin + 1, end);
            exchange(a, begin, i);
        }
    }
}

pub fn test_permutations() {
    let mut a = [1, 2, 3];
    let len = a.len();

    permutations(&mut a, 0, len);
}
