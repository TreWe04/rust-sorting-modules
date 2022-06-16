#![allow(while_true)]

fn make_arr2 (arr: &Vec<i32>) -> Vec<String> {
    let mut result = Vec::new();
    for num in arr {
        result.push(num.to_string());
    }
    result
}

pub fn visualize(arr: &[i32], instructions: Vec<String>) -> Vec<i32> {
    let mut arr1: Vec<i32> = arr.to_vec();
    let mut arr2: Vec<String> = make_arr2(&arr1);

    println!("{}", &arr2.join(", "));
    println!("Steps to sort: {}",  instructions.len());
    println!("Key: \x1b[102mInitial Value\x1b[0m, \x1b[103mCompared Value\x1b[0m, \x1b[101mSwapped Values\x1b[0m");

    for instruction in &instructions {
        let action: Vec<&str> = instruction.split(" ").collect();
        match action[0] {
            "cmp" => {
                let index1 = action[1].parse::<usize>().unwrap();
                let index2 = action[2].parse::<usize>().unwrap();

                arr2 = make_arr2(&arr1);
                arr2[index1] = format!("\x1b[102m{}\x1b[0m", arr2[index1]);
                arr2[index2] = format!("\x1b[103m{}\x1b[0m", arr2[index2]);
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

pub fn selection_sort(arr: &[i32]) -> Vec<String> {
    if arr.len() == 0 {return vec![]};
    let mut arr1: Vec<i32> = arr.to_vec();
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

pub fn bubble_sort(arr: &[i32]) -> Vec<String> {
    if arr.len() == 0 {return vec![]};
    let mut arr1: Vec<i32> = arr.to_vec();
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

pub fn insertion_sort(arr: &[i32]) -> Vec<String> {
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