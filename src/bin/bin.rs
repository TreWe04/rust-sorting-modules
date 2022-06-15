#![allow(unused_imports)]
use sorting_modules::{visualizations, benchmarks};

fn main() {
    let test_arr = [1, 1, 4, 3, 2, 6, 4, 5, 7, 40, 2, 4, 5, 8, 9];
    visualizations::visualize(&test_arr, visualizations::selection_sort(&test_arr));
}