pub mod benchmarks;

pub mod visualizations;

#[cfg(test)]
mod tests {
    static TC1: [i32; 0] = [];
    static TC2: [i32; 1]  = [1];
    static TC3: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    static TC4: [i32; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    static TC5: [i32; 12] = [5, 5, 4, 4, 3, 3, 2, 2, 1, 1, 0, 0];
    static TC6: [i32; 10] = [10, 90, 20, 80, 30, 70, 40, 60, 50, 50];

    #[test]
    fn selection_sort_benchmark() {
        assert_eq!(crate::benchmarks::selection_sort(&TC1), []);
        assert_eq!(crate::benchmarks::selection_sort(&TC2), [1]);
        assert_eq!(crate::benchmarks::selection_sort(&TC3), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::benchmarks::selection_sort(&TC4), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::benchmarks::selection_sort(&TC5), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(crate::benchmarks::selection_sort(&TC6), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn selection_sort_visual() {
        assert_eq!(crate::visualizations::visualize(&TC1, crate::visualizations::selection_sort(&TC1)), []);
        assert_eq!(crate::visualizations::visualize(&TC2, crate::visualizations::selection_sort(&TC2)), [1]);
        assert_eq!(crate::visualizations::visualize(&TC3, crate::visualizations::selection_sort(&TC3)), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::visualizations::visualize(&TC4, crate::visualizations::selection_sort(&TC4)), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::visualizations::visualize(&TC5, crate::visualizations::selection_sort(&TC5)), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(crate::visualizations::visualize(&TC6, crate::visualizations::selection_sort(&TC6)), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn bubble_sort_benchmark() {
        assert_eq!(crate::benchmarks::bubble_sort(&TC1), []);
        assert_eq!(crate::benchmarks::bubble_sort(&TC2), [1]);
        assert_eq!(crate::benchmarks::bubble_sort(&TC3), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::benchmarks::bubble_sort(&TC4), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::benchmarks::bubble_sort(&TC5), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(crate::benchmarks::bubble_sort(&TC6), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn bubble_sort_visual() {
        assert_eq!(crate::visualizations::visualize(&TC1, crate::visualizations::bubble_sort(&TC1)), []);
        assert_eq!(crate::visualizations::visualize(&TC2, crate::visualizations::bubble_sort(&TC2)), [1]);
        assert_eq!(crate::visualizations::visualize(&TC3, crate::visualizations::bubble_sort(&TC3)), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::visualizations::visualize(&TC4, crate::visualizations::bubble_sort(&TC4)), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::visualizations::visualize(&TC5, crate::visualizations::bubble_sort(&TC5)), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(crate::visualizations::visualize(&TC6, crate::visualizations::bubble_sort(&TC6)), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn insertion_sort_benchmark() {
        assert_eq!(crate::benchmarks::insertion_sort(&TC1), []);
        assert_eq!(crate::benchmarks::insertion_sort(&TC2), [1]);
        assert_eq!(crate::benchmarks::insertion_sort(&TC3), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::benchmarks::insertion_sort(&TC4), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::benchmarks::insertion_sort(&TC5), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(crate::benchmarks::insertion_sort(&TC6), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn insertion_sort_visual() {
        assert_eq!(crate::visualizations::visualize(&TC1, crate::visualizations::insertion_sort(&TC1)), []);
        assert_eq!(crate::visualizations::visualize(&TC2, crate::visualizations::insertion_sort(&TC2)), [1]);
        assert_eq!(crate::visualizations::visualize(&TC3, crate::visualizations::insertion_sort(&TC3)), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::visualizations::visualize(&TC4, crate::visualizations::insertion_sort(&TC4)), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::visualizations::visualize(&TC5, crate::visualizations::insertion_sort(&TC5)), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(crate::visualizations::visualize(&TC6, crate::visualizations::insertion_sort(&TC6)), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn heap_sort_benchmark() {
        assert_eq!(crate::benchmarks::heap_sort(&TC1), []);
        assert_eq!(crate::benchmarks::heap_sort(&TC2), [1]);
        assert_eq!(crate::benchmarks::heap_sort(&TC3), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::benchmarks::heap_sort(&TC4), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::benchmarks::heap_sort(&TC5), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(crate::benchmarks::heap_sort(&TC6), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn heap_sort_visual() {
        assert_eq!(crate::visualizations::visualize(&TC1, crate::visualizations::heap_sort(&TC1)), []);
        assert_eq!(crate::visualizations::visualize(&TC2, crate::visualizations::heap_sort(&TC2)), [1]);
        assert_eq!(crate::visualizations::visualize(&TC3, crate::visualizations::heap_sort(&TC3)), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::visualizations::visualize(&TC4, crate::visualizations::heap_sort(&TC4)), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(crate::visualizations::visualize(&TC5, crate::visualizations::heap_sort(&TC5)), [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(crate::visualizations::visualize(&TC6, crate::visualizations::heap_sort(&TC6)), [10, 20, 30, 40, 50, 50, 60, 70, 80, 90]);
    }
}


