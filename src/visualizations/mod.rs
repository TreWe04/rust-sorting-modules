#![allow(while_true)]

fn make_arr2(arr: &Vec<u32>) -> Vec<String> {
    let mut result = Vec::new();
    for num in arr {
        result.push(num.to_string());
    }
    result
}

pub fn visualize<F>(arr: &[u32], algorithm: F) -> Vec<u32> 
    where F: Fn(&[u32]) -> Vec<String>
{
    let instructions: Vec<String> = algorithm(arr);
    let mut arr1: Vec<u32> = arr.to_vec();
    let mut arr2: Vec<String> = make_arr2(&arr1);

    println!("Initial list: {}", &arr2.join(", "));
    println!("Steps to sort: {}",  instructions.len());
    println!("Key: \x1b[102mInitial Value\x1b[0m, \x1b[103mCompared Value\x1b[0m, \x1b[101mSwapped Values\x1b[0m\n");

    for instruction in &instructions {
        let action: Vec<&str> = instruction.split(" ").collect();
        match action[0] {
            "cmp" => {
                let index1 = action[1].parse::<usize>().unwrap();

                arr2 = make_arr2(&arr1);
                arr2[index1] = format!("\x1b[102m{}\x1b[0m", arr2[index1]);
                for i in 2..action.len() {
                    let index = action[i].parse::<usize>().unwrap();
                    arr2[index] = format!("\x1b[103m{}\x1b[0m", arr2[index]);
                }
                println!("{}", &arr2.join(", "))
            },
            "swp" => {
                let index1 = action[1].parse::<usize>().unwrap();
                let index2 = action[2].parse::<usize>().unwrap();
                
                arr1.swap(index1, index2);
                arr2 = make_arr2(&arr1);
                arr2[index1] = format!("\x1b[101m{}\x1b[0m", arr2[index1]);
                arr2[index2] = format!("\x1b[101m{}\x1b[0m", arr2[index2]);
                println!("{}", &arr2.join(", "))
            },
            _ => {println!("Visualization failed"); return arr1;} 
        }
    }

    arr1
}

pub fn selection_sort(arr: &[u32]) -> Vec<String> {
    if arr.len() == 0 {return vec![]};
    let mut arr1: Vec<u32> = arr.to_vec();
    let mut result: Vec<String> = Vec::new();
    
    let mut index = 0;
    for _ in 0..(arr1.len()-1) {
        let mut comparison = arr1[index];
        let mut min_index = index;
        for i in (index+1)..arr1.len() {
            result.push(format!("cmp {} {}", min_index, i));
            if arr1[i] < comparison {
                comparison = arr1[i];
                min_index = i
            }
        }

        if min_index != index {
            arr1.swap(index, min_index);
            result.push(format!("swp {} {}", min_index, index));
        }
        index += 1;
    }

    result
}

pub fn bubble_sort(arr: &[u32]) -> Vec<String> {
    if arr.len() == 0 {return vec![]};
    let mut arr1: Vec<u32> = arr.to_vec();
    let mut result: Vec<String> = Vec::new();

    while true {
        let mut has_swapped = false;

        for i in 0..(arr1.len()-1){
            result.push(format!("cmp {} {}", i, i+1));
            if arr1[i] > arr1[i+1] {
                arr1.swap(i, i+1);
                result.push(format!("swp {} {}", i, i+1));
                has_swapped = true;
            }
        }

        if !has_swapped {
            break;
        }
    }

    result
}

pub fn insertion_sort(arr: &[u32]) -> Vec<String> {
    let mut arr1 = arr.to_vec();
    let mut result: Vec<String> = Vec::new();

    //usize to isize conversion to prevent overflow errors.
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = (i-1) as isize;
        
        while j >= 0 && key < arr1[j as usize] {
            result.push(format!("cmp {} {}", j, j+1));
            arr1.swap(j as usize, (j as usize)+1);
            result.push(format!("swp {} {}", j, j+1));
            j -= 1;
        }
    }

    result
}

pub fn heap_sort(arr: &[u32]) -> Vec<String> {
    fn heapify(result: &mut Vec<String>, arr: &mut Vec<u32>, n: usize, i: usize) {
        // collect indeces of root and children
        let mut largest = i;
        let left_child = i*2 + 1;
        let right_child = i*2 + 2;

        if left_child < n {
            result.push(format!("cmp {} {}", i, left_child));
            
            if arr[left_child] > arr[largest] {
                largest = left_child;
            }
        } 
        if right_child < n {
            result.push(format!("cmp {} {}", i, right_child));

            if arr[right_child] > arr[largest] {
                largest = right_child;
            }
        }

        if largest != i {
            arr.swap(i, largest);
            result.push(format!("swp {} {}", i, largest));
            heapify(result, arr, n, largest);
        }
    }

    let mut arr1 = arr.to_vec();
    let mut result: Vec<String> = Vec::new();

    for i in (0..(arr.len()/2)).rev() {
        heapify(&mut result, &mut arr1, arr.len(), i);
    }

    for i in (0..arr1.len()).rev() {
        arr1.swap(0, i);
        result.push(format!("swp {} {}", 0, i));
        heapify(&mut result, &mut arr1, i, 0);
    }

    result
}

pub fn quick_sort(arr: &[u32]) -> Vec<String> {
    fn pivot (result: &mut Vec<String>, arr: &mut Vec<u32>, low: usize, high: usize) {
        if low >= high {return;}
        let comp = arr[high];
        let mut i = low;

        for j in low..high {
            result.push(format!("cmp {} {} {}", high, i, j));
            if arr[j] <= comp {
                if i != j {
                    arr.swap(i, j);
                    result.push(format!("swp {} {}", i, j));
                }
                i += 1;
            }
        }

        let pi = i;
        arr.swap(pi, high);
        result.push(format!("swp {} {}", pi, high));

        if pi > 0{
            pivot(result, arr, low, pi - 1);
        }
        pivot(result, arr, pi + 1, high);
    }

    if arr.len() == 0 {return [].to_vec();}
    
    let mut arr1 = arr.to_vec();
    let mut result: Vec<String> = Vec::new();

    pivot(&mut result, &mut arr1, 0, arr.len() - 1);

    result
}


pub fn merge_sort(arr: &[u32]) -> Vec<String> {
    fn merge(result: &mut Vec<String>, arr: &mut Vec<u32>, mut  start_l: usize, end: usize){
        
        let mut mid = (start_l + end) / 2;
        println!("{}, {}, {}", start_l, mid, end);
        if start_l < end {
            merge(result, arr, start_l, mid);
            merge(result, arr, mid + 1, end);
        }

        let mut start_r = mid + 1;
        
        if start_r > end || arr[mid] <= arr[start_r] {
            return
        }


        while start_l <= mid && start_r <= end {
            result.push(format!("cmp {} {}", start_r, start_l));
            if arr[start_l] <= arr[start_r] {
                start_l += 1;
            } else {
                let value = arr[start_r];
                let mut index = start_r;

                while index != start_l {
                    result.push(format!("swp {} {}", index, index - 1));
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
    let mut result: Vec<String> = Vec::new();
    merge(&mut result, &mut arr1, 0, arr.len() - 1);
    result
}
