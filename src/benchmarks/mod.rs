#![allow(while_true)]

pub fn selection_sort(arr: &[u32]) -> Vec<u32> {
    if arr.len() == 0 {return vec![]};
    let mut arr1: Vec<u32> = arr.to_vec();
    
    let mut index = 0;
    for _ in 0..(arr1.len()-1) {
        let mut comparison = arr1[index];
        let mut min_index = index;

        for i in (index+1)..arr1.len() {
            if arr1[i] < comparison {
                comparison = arr1[i];
                min_index = i
            }
        }

        if min_index != index {
            arr1.swap(index, min_index);
        }

        index += 1;
    }

    arr1
}


pub fn bubble_sort(arr: &[u32]) -> Vec<u32> {
    if arr.len() == 0 {return vec![]};
    let mut arr1: Vec<u32> = arr.to_vec();

    while true {
        let mut has_swapped = false;

        for i in 0..(arr1.len()-1){
            if arr1[i] > arr1[i+1] {
                arr1.swap(i, i+1);
                has_swapped = true;
            }
        }

        if !has_swapped {
            break;
        }
    }

    arr1
}

pub fn insertion_sort(arr: &[u32]) -> Vec<u32> {
    let mut arr1 = arr.to_vec();

    //j is internally stored as isize conversion to prevent overflow errors.
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = (i-1) as isize;
        while j >= 0 && key < arr1[j as usize] {
            arr1.swap(j as usize, (j as usize)+1);
            j -= 1;
        }
    }

    arr1
}

pub fn heap_sort(arr: &[u32]) -> Vec<u32> {
    fn heapify(arr: &mut Vec<u32>, n: usize, i: usize) {
        // collect indeces of root and children
        let mut largest = i;
        let left_child = i*2 + 1;
        let right_child = i*2 + 2;

        if left_child < n && arr[left_child] > arr[largest] {
            largest = left_child;
        } 
        
        if right_child < n && arr[right_child] > arr[largest] {
            largest = right_child;
        }

        if largest != i {
            arr.swap(i, largest);
            heapify(arr, n, largest);
        }
    }

    let mut arr1 = arr.to_vec();

    for i in (0..(arr.len()/2)).rev() {
        heapify(&mut arr1, arr.len(), i);
    }

    for i in (0..arr1.len()).rev() {
        arr1.swap(0, i);
        heapify(&mut arr1, i, 0);
    }

    arr1
}

pub fn quick_sort(arr: &[u32]) -> Vec<u32> {
    fn pivot (arr: &mut Vec<u32>, low: usize, high: usize) {
        if low >= high {return;}
        let comp = arr[high];
        let mut i = low;

        for j in low..high {
            if arr[j] <= comp {
                arr.swap(i, j);
                i += 1;
            }
        }

        let pi = i;
        arr.swap(pi, high);

        if pi > 0{
            pivot(arr, low, pi - 1);
        }
        pivot(arr, pi + 1, high);

    }

    if arr.len() == 0 {return [].to_vec();}
    
    let mut arr1 = arr.to_vec();

    pivot(&mut arr1, 0, arr.len() - 1);

    arr1
}

pub fn merge_sort(arr: &[u32]) -> Vec<u32> {
    fn merge(arr: &mut Vec<u32>, mut  start_l: usize, end: usize){
        
        let mut mid = (start_l + end) / 2;
        println!("{}, {}, {}", start_l, mid, end);
        if start_l < end {
            merge(arr, start_l, mid);
            merge(arr, mid + 1, end);
        }

        let mut start_r = mid + 1;
        
        if start_r > end || arr[mid] <= arr[start_r] {
            return
        }


        while start_l <= mid && start_r <= end {
            if arr[start_l] <= arr[start_r] {
                start_l += 1;
            } else {
                let value = arr[start_r];
                let mut index = start_r;

                while index != start_l {
                    arr[index] = arr[index - 1];
                    index -= 1;
                }

                arr[start_l] = value;

                start_l += 1;
                mid += 1;
                start_r += 1;
            }
        }   
    }
    
    if arr.len() == 0 {return [].to_vec();}

    let mut arr1 = arr.to_vec();
    merge(&mut arr1, 0, arr.len() - 1);
    arr1
}
