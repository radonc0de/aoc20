use std::fs;

fn main() {

    //turns puzzle input into vector of &str
    let input = fs::read_to_string("input")
        .expect("Failed to read 'input'");
    let input_to_str = input.lines().collect::<Vec<&str>>();

}

fn solver() {



}
