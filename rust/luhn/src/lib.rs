/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");

    //strings whos length < 1 = false
    //trim all spaces before analyzing
    let mut digits = code.trim().to_string();
    let mut revdigits: String = digits.chars().rev().collect();
    
    //no non-digit numbers allowed

    //Step 1
    // Double every second digit starting from the right
    //if you double the numbers and its greater than 9, subtract 9 from it.
    //sum all the numbers
    //if evenly divisable by 10, true else false

    false
}
 