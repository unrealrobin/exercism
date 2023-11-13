use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;

//https://docs.rs/time/latest/time/
// 1 Gigasecond =  1 billion seconds


// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    //unimplemented!("What time is a gigasecond later than {start}");
    let giga: i64 = 1000000000;

    start.checked_add(giga.seconds()).unwrap()
   
}


// From some Data, Find a Date and time 1 gigasecond after that