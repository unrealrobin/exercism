pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // unimplemented!(
    //     "Using the binary search algorithm, find the element '{key}' in the array '{array:?}' and return its index."

    // );

    /* Find the half Way point of the vector or array'
    Compare the key with that element
    if the key is greater than that element repeat process with 2nd half
    if the key is less than that element repeat process with 1st half
    Continue until key is found
    return as an option the index of the found key element match */

    //let length_of_array = array.len();
    let start = 0;
    let end = array.len() - 1;
    let index: Option<usize> = recurse(array, key, start, end);
    index
    
}

pub fn recurse (array: &[i32], key: i32, start: usize, end:usize) -> Option<usize>{

    let index;
    let mid = start + (end - start) / 2;

    if key == array[mid]{
        index = Some(mid);
        return index;
    }

    match key {
        
        key if key < array[mid] => {
            index = recurse(array, key, 0, mid);
            return index;
          
        },
        key if key > array[mid] => {
            index = recurse(array, key, mid, end);
            return index;
        },
        _ => None,
    }

}