pub fn selection_sort (arr: &[i32]) -> Vec<i32>{
    let mut arr1: Vec<i32> = arr.to_vec();
    
    let mut index = 0;
    for _ in 0..(arr1.len()-1) {
        let mut comparison = arr1[index];
        let mut temp_index = index;
        for i in (index+1)..arr1.len() {
            if arr1[i] < comparison {
                comparison = arr1[i];
                temp_index = i
            }
        }

        if temp_index != index {
            arr1.swap(index, temp_index);
        }
        index += 1;
    }

    arr1
}