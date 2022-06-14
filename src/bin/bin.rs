use sorting_modules;

fn main() {
    let test_arr = [];
    println!("{:?} sorted is {:?}", test_arr, sorting_modules::benchmarks::selection_sort(&test_arr));
}