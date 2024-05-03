mod binary_search;
mod sort;
mod hanoi;
mod n_queen;
mod permutations;

use hanoi::test_hanoi;
use permutations::test_permutations;

fn main() {
    test_hanoi();
    test_permutations();
}
