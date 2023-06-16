use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // let allchars: Vec<char> = input.chars().collect();
    // let mut reversed_string_vec: Vec<char> = vec![];

    // println!("{:?}", allchars);

    // for c in allchars {
    //     reversed_string_vec.insert(0, c);
    // }

    
    // let reversed_string:String = reversed_string_vec.iter().collect();
    
    // println!("{}", reversed_string);



    let reversed_string = input.graphemes(true).rev().collect();
    println!("{}", reversed_string);
    reversed_string

}