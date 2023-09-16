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

    // if array is empty then None
    if array.len() == 0 {
        return None;
    } 

    match key {
        key if key == array[0] => {
            return Some(0)
        },
        key if key == array[array.len() - 1] => {
            return Some(array.len() - 1)
        },
        _ => recurse(array, key, 0, array.len() - 1)
    }

    // recurse(array, key, 0, array.len() - 1)
  
    
}

pub fn recurse (array: &[i32], key: i32, start: usize, end:usize) -> Option<usize>{

    let index;
    let mid = start + (end - start) / 2;
    let value_at_mid = array[mid];

    //Base Case
    if key == value_at_mid{
        return Some(mid);
    }

    //Checks for Crossing of bounds
    if  start == mid || mid == end || start == end {
        if key == array[mid]{
            return Some(mid);
        }else{
            return None;
        }
    }

    match key {
        
        key if key < array[mid] => {
            index = recurse(array, key, 0, mid);
            return index;
          
        },
        key if key > array[mid] => {
            index = recurse(array, key, mid + 1, end);
            return index;
        },
        _ => None,
    }

}