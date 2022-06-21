pub mod benchmarks;

pub mod visualizations;

#[cfg(test)]
mod tests {
    use crate::benchmarks;
    use crate::visualizations::{self, visualize};
    
    static TC1: [i32; 0] = [];
    static TC2: [i32; 1]  = [1];
    static TC3: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    static TC4: [i32; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    static TC5: [i32; 12] = [5, 5, 4, 4, 3, 3, 2, 2, 1, 1, 0, 0];
    static TC6: [i32; 10] = [10, 90, 20, 80, 30, 70, 40, 60, 50, 50];

    #[test]
    fn selection_sort_benchmark() {
        assert_eq!(benchmarks::selection_sort(&TC1), []);
        assert_eq!(benchmarks::selection_sort(&TC2), [1]);
        assert_eq!(benchmarks::selection_sort(&TC3), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::selection_sort(&TC4), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::selection_sort(&TC5), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(benchmarks::selection_sort(&TC6), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn selection_sort_visual() {
        assert_eq!(visualize(&TC1, visualizations::selection_sort), []);
        assert_eq!(visualize(&TC2, visualizations::selection_sort), [1]);
        assert_eq!(visualize(&TC3, visualizations::selection_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC4, visualizations::selection_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC5, visualizations::selection_sort), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(visualize(&TC6, visualizations::selection_sort), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn bubble_sort_benchmark() {
        assert_eq!(benchmarks::bubble_sort(&TC1), []);
        assert_eq!(benchmarks::bubble_sort(&TC2), [1]);
        assert_eq!(benchmarks::bubble_sort(&TC3), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::bubble_sort(&TC4), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::bubble_sort(&TC5), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(benchmarks::bubble_sort(&TC6), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn bubble_sort_visual() {
        assert_eq!(visualize(&TC1, visualizations::bubble_sort), []);
        assert_eq!(visualize(&TC2, visualizations::bubble_sort), [1]);
        assert_eq!(visualize(&TC3, visualizations::bubble_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC4, visualizations::bubble_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC5, visualizations::bubble_sort), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(visualize(&TC6, visualizations::bubble_sort), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn insertion_sort_benchmark() {
        assert_eq!(benchmarks::insertion_sort(&TC1), []);
        assert_eq!(benchmarks::insertion_sort(&TC2), [1]);
        assert_eq!(benchmarks::insertion_sort(&TC3), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::insertion_sort(&TC4), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::insertion_sort(&TC5), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(benchmarks::insertion_sort(&TC6), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn insertion_sort_visual() {
        assert_eq!(visualize(&TC1, visualizations::insertion_sort), []);
        assert_eq!(visualize(&TC2, visualizations::insertion_sort), [1]);
        assert_eq!(visualize(&TC3, visualizations::insertion_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC4, visualizations::insertion_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC5, visualizations::insertion_sort), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(visualize(&TC6, visualizations::insertion_sort), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn heap_sort_benchmark() {
        assert_eq!(benchmarks::heap_sort(&TC1), []);
        assert_eq!(benchmarks::heap_sort(&TC2), [1]);
        assert_eq!(benchmarks::heap_sort(&TC3), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::heap_sort(&TC4), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::heap_sort(&TC5), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(benchmarks::heap_sort(&TC6), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn heap_sort_visual() {
        assert_eq!(visualize(&TC1, visualizations::heap_sort), []);
        assert_eq!(visualize(&TC2, visualizations::heap_sort), [1]);
        assert_eq!(visualize(&TC3, visualizations::heap_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC4, visualizations::heap_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC5, visualizations::heap_sort), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(visualize(&TC6, visualizations::heap_sort), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn quick_sort_benchmark() {
        assert_eq!(benchmarks::quick_sort(&TC1), []);
        assert_eq!(benchmarks::quick_sort(&TC2), [1]);
        assert_eq!(benchmarks::quick_sort(&TC3), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::quick_sort(&TC4), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::quick_sort(&TC5), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(benchmarks::quick_sort(&TC6), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn quick_sort_visual() {
        assert_eq!(visualize(&TC1, visualizations::quick_sort), []);
        assert_eq!(visualize(&TC2, visualizations::quick_sort), [1]);
        assert_eq!(visualize(&TC3, visualizations::quick_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC4, visualizations::quick_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC5, visualizations::quick_sort), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(visualize(&TC6, visualizations::quick_sort), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn merge_sort_benchmark() {
        assert_eq!(benchmarks::merge_sort(&TC1), []);
        assert_eq!(benchmarks::merge_sort(&TC2), [1]);
        assert_eq!(benchmarks::merge_sort(&TC3), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::merge_sort(&TC4), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(benchmarks::merge_sort(&TC5), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(benchmarks::merge_sort(&TC6), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn merge_sort_visual() {
        assert_eq!(visualize(&TC1, visualizations::merge_sort), []);
        assert_eq!(visualize(&TC2, visualizations::merge_sort), [1]);
        assert_eq!(visualize(&TC3, visualizations::merge_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC4, visualizations::merge_sort), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(visualize(&TC5, visualizations::merge_sort), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(visualize(&TC6, visualizations::merge_sort), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }
}


