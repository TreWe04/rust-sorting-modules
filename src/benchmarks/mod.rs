#![allow(while_true)]

pub fn selection_sort (arr: &[i32]) -> Vec<i32> {
    if arr.len() == 0 {return vec![]};
    let mut arr1: Vec<i32> = arr.to_vec();
    
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


pub fn bubble_sort (arr: &[i32]) -> Vec<i32> {
    if arr.len() == 0 {return vec![]};
    let mut arr1: Vec<i32> = arr.to_vec();

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