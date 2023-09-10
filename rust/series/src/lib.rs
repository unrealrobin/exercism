pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut store:Vec<String> = vec![]; // return vector with sub strings
    let digits_iter:Vec<String> = digits.chars().map(|c| c.to_string()).collect(); // Output: ["h", "e", "l", "l", "o"]
    let mut digits_len = digits.len() as i32; // length of the vector
    let mut i = 0;

    // if digits.chars().collect().len() == 0 {
    //     return store;
    // }

    while digits_len >= 0 {

        if digits_len < (len as i32) {
            break;
        }

        store.push(digits_iter[i..len+i].join(""));


        i+=1;
        digits_len -= 1;
    }



    store
}
