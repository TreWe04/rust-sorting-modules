#![allow(unused_imports)]
use sorting_modules::{visualizations, benchmarks};

fn main() {
    let test_arr = [8, 92, 57, 64, 20, 20, 59, 28, 4, 4, 35];
    println!("{:?}", benchmarks::heap_sort(&test_arr));
    visualizations::visualize(&test_arr, visualizations::heap_sort(&test_arr));
}