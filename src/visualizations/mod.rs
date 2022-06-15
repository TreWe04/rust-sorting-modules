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
    println!("{}\nSteps to sort: {}", &arr2.join(", "), instructions.len());

    for instruction in &instructions {
        let action: Vec<&str> = instruction.split(" ").collect();
        match action[0] {
            "cmp" => {
                arr2 = make_arr2(&arr1);
                let index1 = action[1].parse::<usize>().unwrap();
                let index2 = action[2].parse::<usize>().unwrap();
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

pub fn selection_sort (arr: &[i32]) -> Vec<String> {
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