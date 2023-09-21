pub fn square(s: u32) -> u64 {

    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }

    let base: u64 = 2;
    let total: u64 = base.pow(s - 1);
    total
}

pub fn total() -> u64 { 
    let mut total: u64 = 0;
    let mut index:u32 = 1;

    while index <= 64 {
        total += square(index) as u64;
        index+=1;
    }
    

    total
}
