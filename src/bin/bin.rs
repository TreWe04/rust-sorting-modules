#![allow(unused_imports)]
use sorting_modules::{visualizations, benchmarks};

fn main() {
    let test_arr = [93, 92, 57, 64, 20, 20, 59, 28, 4, 4, 35];
    println!("{:?}", benchmarks::insertion_sort(&test_arr));
    // visualizations::visualize(&test_arr, visualizations::selection_sort(&test_arr));
}