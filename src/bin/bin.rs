#![allow(unused_imports)]
use sorting_modules::{visualizations, benchmarks};

fn main() {
    let _test_arr = [8, 92, 57, 64, 20, 20, 59, 28, 4, 4, 35];
    // println!("{:?}", benchmarks::merge_sort(&_test_arr));
    visualizations::visualize(&_test_arr, visualizations::merge_sort);
}