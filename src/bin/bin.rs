use sorting_modules::{visualizations, benchmarks};

fn main() {
    let test_arr = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    visualizations::visualize(&test_arr, visualizations::selection_sort(&test_arr));
}