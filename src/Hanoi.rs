
fn move_to(fr: char, to: char) {
    println!("Move: {} -> {}", fr, to);
}

fn solve_hanoi(n: usize, fr: char, to: char, temp: char) {
    if n == 1 {             // 只有一个，从fr移动到to
        move_to(fr, to);
    } else {
        solve_hanoi(n - 1, fr, temp, to);   // 将上面的n-1个移动到辅助的柱子temp
        move_to(fr, to);                       // 将最后一个移动到目的柱子to
        solve_hanoi(n - 1, temp, to, fr);    // 把在辅助柱子temp的n-1个移动到目的柱子to
    }
}

pub(crate) fn test_hanoi() {
    let n = 3;
    solve_hanoi(n, 'a', 'b', 'c');
}