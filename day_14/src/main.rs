
use std::fs;

fn main() {

    let input = fs::read_to_string("input")
        .expect("Failed to read 'input'");
    let lines = input.lines().collect::<Vec<&str>>();
    
    println!("{:?}", lines);

}


## create an array of 36 length that contains Option<bool> values 
 that becomes the bitmask, then create a vector and write to addresses of it with each line using the specified bitmask at the time 
sdfsfs, at the end iterate through the huge vector and calculate the sum at the end
